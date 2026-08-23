mod common;
mod config;
mod proxy;
use crate::config::Config;
use crate::proxy::*;
use std::collections::HashMap;
use serde_json::Value;
use uuid::Uuid;
use worker::*;
use once_cell::sync::Lazy;
use regex::Regex;
static PROXYIP_PATTERN: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"^.+-\d+$").unwrap());
static PROXYKV_PATTERN: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"^([A-Z]{2})$").unwrap());
const BASE_DOMAIN: &str = "xrayjwf.dpdns.org";
const PROXY_JSON_URL: &str =
    "https://raw.githubusercontent.com/jwfathoni17/xrayjwf/main/proxy.json";
// WEB UI
const WEB_UI: &str = r###"<!DOCTYPE html>
<html lang="en">
<head>
<meta charset="UTF-8">
<meta name="viewport" content="width=device-width,initial-scale=1">
<title>XRAYJWF</title>
<style>
* {
    box-sizing: border-box;
}
body {
    margin: 0;
    background: #f5f5f5;
    color: #111;
    font-family:
        Inter,
        system-ui,
        -apple-system,
        BlinkMacSystemFont,
        "Segoe UI",
        sans-serif;
}
.container {
    width: min(100% - 24px, 620px);
    margin: auto;
    padding: 32px 0 60px;
}
.header {
    text-align: center;
    margin-bottom: 28px;
}
.logo {
    width: 44px;
    height: 44px;
    margin: auto;
    display: grid;
    place-items: center;
    border: 1px solid #111;
    border-radius: 12px;
    font-weight: 800;
}
.title {
    margin: 14px 0 4px;
    font-size: 22px;
    letter-spacing: -0.03em;
}
.subtitle {
    color: #888;
    font-size: 12px;
}
.card {
    background: #fff;
    border: 1px solid #ddd;
    border-radius: 12px;
    padding: 16px;
    margin-bottom: 12px;
}
.section-title {
    margin-bottom: 12px;
    font-size: 11px;
    font-weight: 700;
    color: #777;
    text-transform: uppercase;
    letter-spacing: .06em;
}
/* SERVER */
.country {
    margin-bottom: 18px;
}
.country:last-child {
    margin-bottom: 0;
}
.country-name {
    margin-bottom: 8px;
    font-size: 12px;
    font-weight: 700;
}
.server {
    width: 100%;
    min-height: 42px;
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 8px 10px;
    margin-bottom: 6px;
    background: #fafafa;
    border: 1px solid #e3e3e3;
    border-radius: 8px;
    cursor: pointer;
    color: #111;
    text-align: left;
}
.server:hover {
    border-color: #aaa;
}
.server.selected {
    border-color: #111;
    background: #f0f0f0;
}
.server-left {
    display: flex;
    align-items: center;
    gap: 9px;
    min-width: 0;
}
.radio {
    width: 13px;
    height: 13px;
    flex: 0 0 13px;
    border: 1px solid #aaa;
    border-radius: 50%;
}
.server.selected .radio {
    border: 4px solid #111;
}
.server-address {
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
    font-family:
        "SFMono-Regular",
        Consolas,
        monospace;
    font-size: 11px;
}
.server-number {
    margin-left: 8px;
    color: #999;
    font-size: 9px;
}
.ping {
    flex-shrink: 0;
    margin-left: 10px;
    font-family:
        "SFMono-Regular",
        Consolas,
        monospace;
    font-size: 10px;
    color: #888;
}
/* WC */
select,
input {
    width: 100%;
    height: 44px;
    padding: 0 12px;
    border: 1px solid #d5d5d5;
    border-radius: 8px;
    background: #fff;
    color: #111;
    outline: none;
}
input {
    display: none;
    margin-top: 10px;
}
/* CREATE */
.create {
    width: 100%;
    height: 44px;
    border: 0;
    border-radius: 8px;
    background: #111;
    color: #fff;
    font-size: 12px;
    font-weight: 700;
    cursor: pointer;
}
.create:hover {
    opacity: .88;
}
.create:disabled {
    opacity: .45;
    cursor: not-allowed;
}
/* CONFIG */
.config {
    display: none;
}
.config.show {
    display: block;
}
.config-title {
    display: flex;
    align-items: center;
    justify-content: space-between;
    margin-bottom: 10px;
    font-size: 11px;
    font-weight: 700;
}
.copy {
    height: 28px;
    padding: 0 10px;
    border: 0;
    border-radius: 6px;
    background: #eee;
    color: #111;
    font-size: 10px;
    font-weight: 700;
    cursor: pointer;
}
pre {
    margin: 0;
    padding: 12px;
    overflow-x: auto;
    background: #f7f7f7;
    border: 1px solid #e7e7e7;
    border-radius: 8px;
    font-family:
        "SFMono-Regular",
        Consolas,
        monospace;
    font-size: 10px;
    line-height: 1.6;
    white-space: pre-wrap;
    word-break: break-all;
}
.status {
    margin-top: 16px;
    color: #888;
    font-size: 11px;
    text-align: center;
}
.footer {
    margin-top: 28px;
    color: #aaa;
    font-size: 10px;
    text-align: center;
}
</style>
</head>
<body>
<div class="container">
<div class="header">
    <div class="logo">
        X
    </div>
    <div class="title">
        XRAYJWF
    </div>
    <div class="subtitle">
        Minimal Configuration Panel
    </div>
</div>
<!-- SERVER -->
<div class="card">
    <div class="section-title">
        Server
    </div>
    <div id="servers">
        Loading servers...
    </div>
</div>
<!-- WC -->
<div class="card">
    <div class="section-title">
        WC
    </div>
    <select id="wc">
        <option value="nonwc">
            NON WC
        </option>
        <option value="support.zoom.us">
            SUPPORT.ZOOM.US
        </option>
        <option value="ava.game.naver">
            AVA.GAME.NAVER
        </option>
        <option value="custom">
            CUSTOM
        </option>
    </select>
    <input
        id="custom"
        type="text"
        placeholder="Enter custom WC"
        autocomplete="off"
    >
</div>
<!-- CREATE -->
<div class="card">
    <button
        id="create"
        class="create"
        disabled
        onclick="createAccount()"
    >
        CREATE ACCOUNT
    </button>
</div>
<!-- VLESS -->
<div
    id="vlessCard"
    class="card config"
>
    <div class="config-title">
        <span>
            VLESS CONFIG
        </span>
        <button
            class="copy"
            onclick="copyText('vless')"
        >
            COPY
        </button>
    </div>
    <pre id="vless"></pre>
</div>
<!-- CLASH -->
<div
    id="clashCard"
    class="card config"
>
    <div class="config-title">
        <span>
            CLASH CONFIG
        </span>
        <button
            class="copy"
            onclick="copyText('clash')"
        >
            COPY
        </button>
    </div>
    <pre id="clash"></pre>
</div>
<div
    id="status"
    class="status"
>
    Loading...
</div>
<div class="footer">
    Xray Worker
</div>
</div>
<script>
let selectedServer = null;
let serverData = {};
const serversElement =
    document.getElementById("servers");
const createButton =
    document.getElementById("create");
const statusElement =
    document.getElementById("status");
const wcElement =
    document.getElementById("wc");
const customElement =
    document.getElementById("custom");
// LOAD SERVER LIST
async function loadServers() {
    try {
        const response =
            await fetch(
                "/api/servers",
                {
                    cache: "no-store"
                }
            );
        if (!response.ok) {
            throw new Error(
                "HTTP " + response.status
            );
        }
        serverData =
            await response.json();
        renderServers();
        statusElement.textContent =
            "Select a server";
    } catch (error) {
        console.error(error);
        serversElement.innerHTML =
            "<div style='font-size:11px;color:#888'>Unable to load servers</div>";
        statusElement.textContent =
            "Failed to load servers";
    }
}
// RENDER SERVER
function renderServers() {
    serversElement.innerHTML = "";
    for (
        const location in serverData
    ) {
        const servers =
            serverData[location];
        const country =
            document.createElement("div");
        country.className =
            "country";
        const title =
            document.createElement("div");
        title.className =
            "country-name";
        title.textContent =
            getCountryName(location);
        country.appendChild(title);
        servers.forEach(
            (server, index) => {
                const button =
                    document.createElement(
                        "button"
                    );
                button.className =
                    "server";
                const left =
                    document.createElement(
                        "div"
                    );
                left.className =
                    "server-left";
                const radio =
                    document.createElement(
                        "div"
                    );
                radio.className =
                    "radio";
                const address =
                    document.createElement(
                        "div"
                    );
                address.className =
                    "server-address";
                address.textContent =
                    server;
                const number =
                    document.createElement(
                        "span"
                    );
                number.className =
                    "server-number";
                number.textContent =
                    "#" + (index + 1);
                address.appendChild(
                    number
                );
                left.appendChild(radio);
                left.appendChild(address);
                const ping =
                    document.createElement(
                        "div"
                    );
                ping.className =
                    "ping";
                ping.textContent =
                    "—";
                button.appendChild(left);
                button.appendChild(ping);
                button.onclick = () => {
                    selectServer(
                        location,
                        index,
                        button
                    );
                };
                country.appendChild(button);
            }
        );
        serversElement.appendChild(
            country
        );
    }
}
// SELECT SERVER
function selectServer(
    location,
    index,
    element
) {
    document
        .querySelectorAll(".server")
        .forEach(
            item =>
                item.classList.remove(
                    "selected"
                )
        );
    element.classList.add(
        "selected"
    );
    selectedServer = {
        location: location,
        index: index + 1
    };
    createButton.disabled =
        false;
    statusElement.textContent =
        getCountryName(location) +
        " / Server " +
        (index + 1) +
        " selected";
}
// WC
wcElement.addEventListener(
    "change",
    () => {
        if (
            wcElement.value ===
            "custom"
        ) {
            customElement.style.display =
                "block";
        } else {
            customElement.style.display =
                "none";
            customElement.value =
                "";
        }
    }
);
// CREATE ACCOUNT
async function createAccount() {
    if (!selectedServer) {
        alert(
            "Please select a server first"
        );
        return;
    }
    if (
        wcElement.value ===
        "custom" &&
        !customElement.value.trim()
    ) {
        alert(
            "Enter custom WC"
        );
        return;
    }
    createButton.disabled =
        true;
    createButton.textContent =
        "GENERATING...";
    statusElement.textContent =
        "Generating configuration...";
    try {
        let url =
            "/link?" +
            "location=" +
            encodeURIComponent(
                selectedServer.location
            ) +
            "&server=" +
            encodeURIComponent(
                selectedServer.index
            ) +
            "&wc=" +
            encodeURIComponent(
                wcElement.value
            );
        if (
            wcElement.value ===
            "custom"
        ) {
            url +=
                "&custom=" +
                encodeURIComponent(
                    customElement.value.trim()
                );
        }
        const response =
            await fetch(
                url,
                {
                    cache: "no-store"
                }
            );
        if (!response.ok) {
            throw new Error(
                "HTTP " +
                response.status
            );
        }
        const text =
            await response.text();
        const parts =
            text.split(
                "\n---CLASH---\n"
            );
        document
            .getElementById("vless")
            .textContent =
                parts[0] || "";
        document
            .getElementById("clash")
            .textContent =
                parts[1] || "";
        document
            .getElementById("vlessCard")
            .classList.add("show");
        document
            .getElementById("clashCard")
            .classList.add("show");
        statusElement.textContent =
            "Configuration generated";
    } catch (error) {
        console.error(error);
        statusElement.textContent =
            "Failed to generate configuration";
    } finally {
        createButton.disabled =
            false;
        createButton.textContent =
            "CREATE ACCOUNT";
    }
}
// COPY
async function copyText(id) {
    const text =
        document
            .getElementById(id)
            .textContent;
    try {
        await navigator
            .clipboard
            .writeText(text);
        statusElement.textContent =
            "Copied";
    } catch {
        statusElement.textContent =
            "Copy failed";
    }
}
// COUNTRY NAME
function getCountryName(code) {
    const names = {
        ID: "🇮🇩 Indonesia",
        SG: "🇸🇬 Singapore",
        US: "🇺🇸 United States",
        DE: "🇩🇪 Germany",
        FR: "🇫🇷 France",
        NL: "🇳🇱 Netherlands",
        GB: "🇬🇧 United Kingdom",
        FI: "🇫🇮 Finland",
        IN: "🇮🇳 India",
        RU: "🇷🇺 Russia",
        RO: "🇷🇴 Romania",
        LV: "🇱🇻 Latvia",
        SK: "🇸🇰 Slovakia",
        AM: "🇦🇲 Armenia",
        ZW: "🇿🇼 Zimbabwe"
    };
    return names[code] || code;
}
loadServers();
</script>
</body>
</html>
"###;
// MAIN
#[event(fetch)]
pub async fn main(
    req: Request,
    env: Env,
    _: Context,
) -> Result<Response> {
    let uuid = env
        .var("UUID")
        .map(|x| {
            Uuid::parse_str(
                &x.to_string()
            )
            .unwrap_or_default()
        })?;
    let host = req
        .url()?
        .host()
        .map(|x| x.to_string())
        .unwrap_or_default();
    let config = Config {
        uuid,
        host,
        proxy_addr:
            BASE_DOMAIN.to_string(),
        proxy_port: 443,
    };
    Router::with_data(config)
        .on("/", web_ui)
        .on_async("/api/servers", api_servers)
        .on("/link", link)
        // /ID/1
        // /ID/2
        // /SG/1
        // /SG/2
        .on_async(
            "/:location/:server",
            selected_tunnel
        )
        // Legacy:
        // /ID
        // /SG
        .on_async(
            "/:proxyip",
            tunnel
        )
        .on_async(
            "/Benxx-Project/:proxyip",
            tunnel
        )
        .run(req, env)
        .await
}
// WEB UI ROUTE
fn web_ui(
    _: Request,
    _: RouteContext<Config>,
) -> Result<Response> {
    Response::from_html(WEB_UI)
}
// API SERVERS
async fn api_servers(
    _: Request,
    cx: RouteContext<Config>,
) -> Result<Response> {
    let kv =
        cx.kv("YUMI")?;
    let mut proxy_kv_str =
        kv.get("proxy_kv")
            .text()
            .await?
            .unwrap_or_default();
    // Kalau KV kosong,
    // ambil dari GitHub milik kita.
    if proxy_kv_str.is_empty() {
        let request =
            Fetch::Url(
                Url::parse(
                    PROXY_JSON_URL
                )?
            );
        let mut response =
            request.send().await?;
        if response.status_code() != 200 {
            return Response::error(
                "Failed to load proxy.json",
                502
            );
        }
        proxy_kv_str =
            response.text().await?;
        kv.put(
            "proxy_kv",
            &proxy_kv_str
        )?
        .expiration_ttl(300)
        .execute()
        .await?;
    }
    let data:
        Value =
        serde_json::from_str(
            &proxy_kv_str
        )?;
    Response::from_json(&data)
}
// SELECTED TUNNEL
//
// /ID/1
// /ID/2
// /ID/3
//
// /SG/1
// /SG/2
// /SG/3
async fn selected_tunnel(
    req: Request,
    mut cx: RouteContext<Config>,
) -> Result<Response> {
    let location =
        cx.param("location")
            .map(|x|
                x.to_uppercase()
            )
            .unwrap_or_default();
    let server_number:
        usize =
        cx.param("server")
            .and_then(
                |x|
                    x.parse::<usize>()
                        .ok()
            )
            .unwrap_or(0);
    if !PROXYKV_PATTERN
        .is_match(&location)
    {
        return Response::error(
            "Invalid location",
            400
        );
    }
    if server_number == 0 {
        return Response::error(
            "Invalid server number",
            400
        );
    }
    let kv =
        cx.kv("YUMI")?;
    let mut proxy_kv_str =
        kv.get("proxy_kv")
            .text()
            .await?
            .unwrap_or_default();
    if proxy_kv_str.is_empty() {
        let request =
            Fetch::Url(
                Url::parse(
                    PROXY_JSON_URL
                )?
            );
        let mut response =
            request.send().await?;
        if response.status_code() != 200 {
            return Response::error(
                "Failed to load proxy.json",
                502
            );
        }
        proxy_kv_str =
            response.text().await?;
        kv.put(
            "proxy_kv",
            &proxy_kv_str
        )?
        .expiration_ttl(300)
        .execute()
        .await?;
    }
    let proxy_kv:
        HashMap<String, Vec<String>> =
        serde_json::from_str(
            &proxy_kv_str
        )?;
    let servers =
        match proxy_kv.get(&location) {
            Some(value) =>
                value,
            None =>
                return Response::error(
                    "Location not found",
                    404
                ),
        };
    // UI menggunakan nomor 1-based.
    //
    // Rust Vec menggunakan index 0-based.
    //
    // /ID/1 -> [0]
    // /ID/2 -> [1]
    // /ID/3 -> [2]
    let index =
        server_number - 1;
    let selected =
        match servers.get(index) {
            Some(value) =>
                value.clone(),
            None =>
                return Response::error(
                    "Server not found",
                    404
                ),
        };
    let proxyip =
        selected.replace(':', "-");
    let upgrade =
        req.headers()
            .get("Upgrade")?
            .unwrap_or_default();
    if upgrade != "websocket" {
        return Response::from_html(
            format!(
                "Xray Worker<br><br>Selected: {}/{}",
                location,
                server_number
            )
        );
    }
    if !PROXYIP_PATTERN
        .is_match(&proxyip)
    {
        return Response::error(
            "Invalid proxy server",
            400
        );
    }
    if let Some(
        (addr, port_str)
    ) =
        proxyip.split_once('-')
    {
        if let Ok(port) =
            port_str.parse::<u16>()
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
                    "[selected-tunnel]: {}",
                    e
                );
            }
        }
    );
    Response::from_websocket(client)
}
// LEGACY / RANDOM TUNNEL
//
// /ID
// /SG
async fn tunnel(
    req: Request,
    mut cx: RouteContext<Config>,
) -> Result<Response> {
    let mut proxyip =
        cx.param("proxyip")
            .map(|s|
                s.to_string()
            )
            .unwrap_or_default();
    if PROXYKV_PATTERN
        .is_match(&proxyip)
    {
        let kvid_list:
            Vec<String> =
            proxyip
                .split(',')
                .map(
                    |s|
                        s.to_string()
                )
                .collect();
        let kv =
            cx.kv("YUMI")?;
        let mut proxy_kv_str =
            kv.get("proxy_kv")
                .text()
                .await?
                .unwrap_or_default();
        let mut rand_buf =
            [0u8; 1];
        getrandom::getrandom(
            &mut rand_buf
        )
        .expect(
            "failed generating random number"
        );
        if proxy_kv_str.is_empty() {
            let request =
                Fetch::Url(
                    Url::parse(
                        PROXY_JSON_URL
                    )?
                );
            let mut response =
                request.send().await?;
            if response.status_code()
                == 200
            {
                proxy_kv_str =
                    response.text()
                        .await?;
                kv.put(
                    "proxy_kv",
                    &proxy_kv_str
                )?
                .expiration_ttl(300)
                .execute()
                .await?;
            } else {
                return Err(
                    Error::from(
                        format!(
                            "error getting proxy kv: {}",
                            response.status_code()
                        )
                    )
                );
            }
        }
        let proxy_kv:
            HashMap<
                String,
                Vec<String>
            > =
            serde_json::from_str(
                &proxy_kv_str
            )?;
        let kv_index =
            (rand_buf[0] as usize)
            % kvid_list.len();
        proxyip =
            kvid_list[kv_index]
                .clone();
        let list =
            match proxy_kv.get(
                &proxyip
            ) {
                Some(value) =>
                    value,
                None =>
                    return Response::error(
                        "Location not found",
                        404
                    ),
            };
        if list.is_empty() {
            return Response::error(
                "No servers available",
                503
            );
        }
        let proxyip_index =
            (rand_buf[0] as usize)
            % list.len();
        proxyip =
            list[proxyip_index]
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
        if let Some(
            (addr, port_str)
        ) =
            proxyip.split_once('-')
        {
            if let Ok(port) =
                port_str.parse::<u16>()
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
        Response::from_websocket(
            client
        )
    } else {
        Response::from_html(
            "Xray Worker"
        )
    }
}
// CONFIG GENERATOR
fn link(
    req: Request,
    cx: RouteContext<Config>,
) -> Result<Response> {
    let url =
        req.url()?;
    let location =
        url.query_pairs()
            .find(
                |(key, _)|
                    key == "location"
            )
            .map(
                |(_, value)|
                    value.to_uppercase()
            )
            .unwrap_or_else(
                ||
                    "ID".to_string()
            );
    let server_number =
        url.query_pairs()
            .find(
                |(key, _)|
                    key == "server"
            )
            .map(
                |(_, value)|
                    value.to_string()
            )
            .unwrap_or_else(
                ||
                    "1".to_string()
            );
    let wc =
        url.query_pairs()
            .find(
                |(key, _)|
                    key == "wc"
            )
            .map(
                |(_, value)|
                    value.to_string()
            )
            .unwrap_or_else(
                ||
                    "nonwc".to_string()
            );
    let custom =
        url.query_pairs()
            .find(
                |(key, _)|
                    key == "custom"
            )
            .map(
                |(_, value)|
                    value.to_string()
            )
            .unwrap_or_default();
    let path =
        format!(
            "/{}/{}",
            location,
            server_number
        );
    let (server, host) =
        build_target(
            &wc,
            &custom
        );
    let uuid =
        cx.data.uuid.to_string();
    // VLESS
    let vless =
        format!(
            "vless://{uuid}@{server}:443\
?encryption=none\
&type=ws\
&host={host}\
&path={}\
&security=tls\
&sni={host}\
#Changli-VLESS",
            urlencoding::encode(&path)
        );
    // CLASH
    let clash =
        format!(
r#"proxies:
  - name: Changli-VLESS
    type: vless
    server: {server}
    port: 443
    uuid: {uuid}
    tls: true
    udp: true
    servername: {host}
    network: ws
    ws-opts:
      path: {path}
      headers:
        Host: {host}"#,
            server = server,
            uuid = uuid,
            host = host,
            path = path
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
// WC TARGET
fn build_target(
    wc: &str,
    custom: &str,
) -> (String, String) {
    if wc.is_empty()
        || wc == "nonwc"
    {
        return (
            BASE_DOMAIN.to_string(),
            BASE_DOMAIN.to_string(),
        );
    }
    let value =
        if wc == "custom" {
            custom.trim()
        } else {
            wc.trim()
        };
    if value.is_empty() {
        return (
            BASE_DOMAIN.to_string(),
            BASE_DOMAIN.to_string(),
        );
    }
    let wildcard_host =
        format!(
            "{}.{}",
            value,
            BASE_DOMAIN
        );
    (
        value.to_string(),
        wildcard_host
    )
}
