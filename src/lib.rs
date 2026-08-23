mod common;
mod config;
mod proxy;

use crate::config::Config;
use crate::proxy::*;

use std::collections::HashMap;

use base64::{engine::general_purpose::URL_SAFE, Engine as _};
use serde_json::json;
use uuid::Uuid;
use worker::*;
use once_cell::sync::Lazy;
use regex::Regex;

static PROXYIP_PATTERN: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"^.+-\d+$").unwrap());

static PROXYKV_PATTERN: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"^([A-Z]{2})").unwrap());

#[event(fetch)]
pub async fn main(req: Request, env: Env, _: Context) -> Result<Response> {
    let uuid = env
        .var("UUID")
        .map(|x| Uuid::parse_str(&x.to_string()).unwrap_or_default())?;

    let host = req
        .url()?
        .host()
        .map(|x| x.to_string())
        .unwrap_or_default();

    let config = Config {
        uuid,
        host: host.clone(),
        proxy_addr: host,
        proxy_port: 443,
    };

    Router::with_data(config)
        .on("/", web_ui)
        .on("/link", link)
        .on_async("/:proxyip", tunnel)
        .on_async("/Benxx-Project/:proxyip", tunnel)
        .run(req, env)
        .await
}


// ============================================================
// WEB UI
// ============================================================

fn web_ui(_: Request, _: RouteContext<Config>) -> Result<Response> {
    let html = r##"<!DOCTYPE html>
<html lang="en">
<head>
<meta charset="UTF-8">
<meta name="viewport" content="width=device-width, initial-scale=1.0">

<title>Xray Panel</title>

<style>
* {
    box-sizing: border-box;
}

html, body {
    margin: 0;
    padding: 0;
    min-height: 100%;
}

body {
    background: #f5f5f5;
    color: #111;
    font-family:
        Inter,
        ui-sans-serif,
        system-ui,
        -apple-system,
        BlinkMacSystemFont,
        "Segoe UI",
        sans-serif;
}

.container {
    width: min(100% - 32px, 680px);
    margin: 0 auto;
    padding: 48px 0 64px;
}

.header {
    text-align: center;
    margin-bottom: 36px;
}

.logo {
    width: 48px;
    height: 48px;
    margin: 0 auto 18px;
    border: 2px solid #111;
    border-radius: 14px;
    display: flex;
    align-items: center;
    justify-content: center;
    font-weight: 800;
    font-size: 18px;
}

.title {
    margin: 0;
    font-size: 24px;
    font-weight: 700;
    letter-spacing: -0.5px;
}

.subtitle {
    margin-top: 7px;
    color: #777;
    font-size: 13px;
}

.card {
    background: #fff;
    border: 1px solid #dedede;
    border-radius: 14px;
    padding: 18px;
    margin-bottom: 14px;
}

.label {
    display: block;
    margin-bottom: 8px;
    font-size: 11px;
    font-weight: 700;
    letter-spacing: .08em;
    text-transform: uppercase;
    color: #777;
}

select {
    width: 100%;
    height: 46px;
    padding: 0 13px;
    border: 1px solid #d5d5d5;
    border-radius: 9px;
    background: #fff;
    color: #111;
    font-size: 14px;
    outline: none;
}

select:focus {
    border-color: #111;
}

button {
    width: 100%;
    height: 46px;
    border: 0;
    border-radius: 9px;
    background: #111;
    color: #fff;
    font-size: 13px;
    font-weight: 700;
    cursor: pointer;
}

button:hover {
    opacity: .88;
}

button:disabled {
    opacity: .5;
    cursor: not-allowed;
}

.config-card {
    display: none;
}

.config-card.show {
    display: block;
}

.config-head {
    display: flex;
    align-items: center;
    justify-content: space-between;
    margin-bottom: 10px;
}

.config-title {
    font-size: 12px;
    font-weight: 700;
    letter-spacing: .06em;
}

.copy {
    width: auto;
    height: 30px;
    padding: 0 12px;
    background: #f1f1f1;
    color: #111;
    font-size: 11px;
}

pre {
    margin: 0;
    padding: 14px;
    background: #f7f7f7;
    border: 1px solid #e7e7e7;
    border-radius: 9px;
    overflow-x: auto;
    white-space: pre-wrap;
    word-break: break-all;
    font-family:
        "SFMono-Regular",
        Consolas,
        "Liberation Mono",
        monospace;
    font-size: 11px;
    line-height: 1.6;
}

.info {
    display: none;
    margin-top: 14px;
}

.info.show {
    display: block;
}

.info-row {
    display: flex;
    justify-content: space-between;
    gap: 20px;
    padding: 9px 0;
    border-bottom: 1px solid #eee;
    font-size: 12px;
}

.info-row:last-child {
    border-bottom: 0;
}

.info-key {
    color: #888;
}

.info-value {
    text-align: right;
    word-break: break-all;
}

.status {
    text-align: center;
    margin-top: 20px;
    font-size: 11px;
    color: #888;
}

.footer {
    text-align: center;
    margin-top: 32px;
    font-size: 11px;
    color: #999;
}

@media (max-width: 480px) {
    .container {
        width: min(100% - 20px, 680px);
        padding-top: 28px;
    }

    .card {
        padding: 14px;
    }
}
</style>
</head>

<body>

<div class="container">

    <div class="header">
        <div class="logo">X</div>

        <h1 class="title">XRAY PANEL</h1>

        <div class="subtitle">
            Cloudflare Worker
        </div>
    </div>


    <div class="card">

        <label class="label">WC</label>

        <select id="wc">
            <option value="nowc">NOWC</option>
            <option value="support">support.zoom.us</option>
        </select>

    </div>


    <div class="card">

        <label class="label">Server</label>

        <div id="server"
             style="
                font-family: monospace;
                font-size: 13px;
                padding: 13px;
                background: #f7f7f7;
                border: 1px solid #e7e7e7;
                border-radius: 9px;
             ">
            Loading...
        </div>

    </div>


    <div class="card">

        <button id="create" onclick="createAccount()">
            CREATE ACCOUNT
        </button>

    </div>


    <div id="vlessCard" class="card config-card">

        <div class="config-head">

            <div class="config-title">
                VLESS CONFIG
            </div>

            <button
                class="copy"
                onclick="copyConfig('vless')">
                COPY
            </button>

        </div>

        <pre id="vless"></pre>

    </div>


    <div id="clashCard" class="card config-card">

        <div class="config-head">

            <div class="config-title">
                CLASH CONFIG
            </div>

            <button
                class="copy"
                onclick="copyConfig('clash')">
                COPY
            </button>

        </div>

        <pre id="clash"></pre>

    </div>


    <div id="info" class="card info">

        <div class="info-row">
            <span class="info-key">UUID</span>
            <span id="uuid" class="info-value"></span>
        </div>

        <div class="info-row">
            <span class="info-key">PATH</span>
            <span class="info-value">/ID</span>
        </div>

        <div class="info-row">
            <span class="info-key">SERVER</span>
            <span id="infoServer" class="info-value"></span>
        </div>

        <div class="info-row">
            <span class="info-key">PORT</span>
            <span class="info-value">443</span>
        </div>

    </div>


    <div id="status" class="status">
        Ready
    </div>


    <div class="footer">
        Xray Worker
    </div>

</div>


<script>

let configs = {
    vless: "",
    clash: ""
};


async function createAccount() {

    const button = document.getElementById("create");

    const status = document.getElementById("status");

    button.disabled = true;
    button.textContent = "GENERATING...";

    status.textContent = "Generating configuration...";

    try {

        const response = await fetch("/link", {
            method: "GET",
            cache: "no-store"
        });

        if (!response.ok) {
            throw new Error("HTTP " + response.status);
        }

        const text = await response.text();

        /*
         * /link format:
         *
         * VLESS
         *
         * CLASH YAML
         */

        const parts = text.split(
            "\n---CLASH---\n"
        );

        const vless = parts[0].trim();

        const clash = parts[1]
            ? parts[1].trim()
            : "";

        configs.vless = vless;
        configs.clash = clash;

        document.getElementById("vless").textContent =
            vless;

        document.getElementById("clash").textContent =
            clash;

        document.getElementById("vlessCard")
            .classList.add("show");

        document.getElementById("clashCard")
            .classList.add("show");

        document.getElementById("info")
            .classList.add("show");

        /*
         * Extract UUID from VLESS
         */

        const match = vless.match(
            /vless:\/\/([^@]+)@/
        );

        if (match) {
            document.getElementById("uuid")
                .textContent = match[1];
        }

        const server =
            window.location.hostname;

        document.getElementById("server")
            .textContent = server;

        document.getElementById("infoServer")
            .textContent = server;

        status.textContent =
            "Account generated";

    } catch (error) {

        console.error(error);

        status.textContent =
            "Failed to generate account";

    } finally {

        button.disabled = false;
        button.textContent = "CREATE ACCOUNT";

    }
}


async function copyConfig(type) {

    const value = configs[type];

    if (!value) return;

    try {

        await navigator.clipboard.writeText(value);

        const status =
            document.getElementById("status");

        status.textContent =
            type === "vless"
            ? "VLESS copied"
            : "Clash configuration copied";

        setTimeout(() => {
            status.textContent =
                "Account generated";
        }, 1500);

    } catch (error) {

        alert("Copy failed");

    }
}


document.getElementById("server")
    .textContent = window.location.hostname;

</script>

</body>
</html>"##;

    Response::from_html(html)
}


// ============================================================
// TUNNEL
// ============================================================

async fn tunnel(
    req: Request,
    mut cx: RouteContext<Config>
) -> Result<Response> {

    let mut proxyip = cx
        .param("proxyip")
        .map(|s| s.to_string())
        .unwrap_or_default();

    if PROXYKV_PATTERN.is_match(&proxyip) {

        let kvid_list: Vec<String> =
            proxyip
                .split(',')
                .map(|s| s.to_string())
                .collect();

        let kv = cx.kv("YUMI")?;

        let mut proxy_kv_str =
            kv.get("proxy_kv")
                .text()
                .await?
                .unwrap_or_default();

        let mut rand_buf = [0u8; 1];

        getrandom::getrandom(&mut rand_buf)
            .expect("failed generating random number");

        if proxy_kv_str.is_empty() {

            console_log!(
                "getting proxy kv from github..."
            );

            let url =
                "https://raw.githubusercontent.com/ziyosen/tunel-worker/refs/heads/main/proxy.json";

            let req =
                Fetch::Url(Url::parse(url)?);

            let mut res =
                req.send().await?;

            if res.status_code() == 200 {

                proxy_kv_str =
                    res.text().await?;

                kv.put(
                    "proxy_kv",
                    &proxy_kv_str
                )?
                .expiration_ttl(60 * 60 * 12)
                .execute()
                .await?;

            } else {

                return Err(
                    Error::from(
                        format!(
                            "error getting proxy kv: {}",
                            res.status_code()
                        )
                    )
                );

            }
        }

        let proxy_kv:
            HashMap<String, Vec<String>> =
            serde_json::from_str(
                &proxy_kv_str
            )?;

        let kv_index =
            (rand_buf[0] as usize)
            % kvid_list.len();

        proxyip =
            kvid_list[kv_index]
                .clone();

        let proxyip_index =
            (rand_buf[0] as usize)
            % proxy_kv[&proxyip].len();

        proxyip =
            proxy_kv[&proxyip]
                [proxyip_index]
                .clone()
                .replace(':', "-");
    }

    let upgrade =
        req.headers()
            .get("Upgrade")?
            .unwrap_or_default();

    if upgrade == "websocket"
        && PROXYIP_PATTERN
            .is_match(&proxyip)
    {

        if let Some((addr, port_str)) =
            proxyip.split_once('-')
        {

            if let Ok(port) =
                port_str.parse()
            {
                cx.data.proxy_addr =
                    addr.to_string();

                cx.data.proxy_port =
                    port;
            }
        }

        let WebSocketPair {
            server,
            client
        } =
            WebSocketPair::new()?;

        server.accept()?;

        wasm_bindgen_futures::spawn_local(
            async move {

                let events =
                    server.events().unwrap();

                if let Err(e) =
                    ProxyStream::new(
                        cx.data,
                        &server,
                        events
                    )
                    .process()
                    .await
                {
                    console_error!(
                        "[tunnel]: {}",
                        e
                    );
                }
            }
        );

        Response::from_websocket(client)

    } else {

        Response::from_html(
            "Xray Worker"
        )

    }
}


// ============================================================
// LINK GENERATOR
// ============================================================

fn link(
    _: Request,
    cx: RouteContext<Config>
) -> Result<Response> {

    let host =
        cx.data.host.to_string();

    let uuid =
        cx.data.uuid.to_string();


    // ------------------------------
    // VLESS
    // ------------------------------

    let vless =
        format!(
            "vless://{uuid}@{host}:443?encryption=none&type=ws&host={host}&path=%2FID&security=tls&sni={host}#Changli-VLESS"
        );


    // ------------------------------
    // CLASH / MIHOMO
    // ------------------------------

    let clash =
        format!(
r#"proxies:
  - name: Changli-VLESS
    type: vless
    server: {host}
    port: 443
    uuid: {uuid}
    tls: true
    udp: true
    servername: {host}
    network: ws
    ws-opts:
      path: /ID
      headers:
        Host: {host}

proxy-groups:
  - name: PROXY
    type: select
    proxies:
      - Changli-VLESS

rules:
  - MATCH,PROXY"#,
            host = host,
            uuid = uuid
        );


    let output =
        format!(
            "{vless}\n---CLASH---\n{clash}"
        );


    Response::from_body(
        ResponseBody::Body(
            output.into()
        )
    )
}
