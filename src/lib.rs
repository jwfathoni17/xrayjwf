mod common;
mod config;
mod proxy;
mod web_ui;

use crate::config::Config;
use crate::proxy::*;
use crate::web_ui::WEB_UI;

use std::collections::HashMap;

use uuid::Uuid;
use worker::*;
use once_cell::sync::Lazy;
use regex::Regex;

static PROXYIP_PATTERN: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"^.+-\d+$").unwrap());

static PROXYKV_PATTERN: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"^([A-Z]{2})").unwrap());

const BASE_DOMAIN: &str = "xrayjwf.dpdns.org";

const WILDCARDS: &[(&str, &str)] = &[
    ("SUPPORT.ZOOM.US", "support.zoom.us"),
    ("AVA.GAME.NAVER", "ava.game.naver"),
];

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
        host,
        proxy_addr: BASE_DOMAIN.to_string(),
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

fn web_ui(_: Request, _: RouteContext<Config>) -> Result<Response> {
    Response::from_html(WEB_UI)
}

async fn tunnel(
    req: Request,
    mut cx: RouteContext<Config>,
) -> Result<Response> {
    let mut proxyip = cx
        .param("proxyip")
        .map(|s| s.to_string())
        .unwrap_or_default();

    if PROXYKV_PATTERN.is_match(&proxyip) {
        let kvid_list: Vec<String> = proxyip
            .split(',')
            .map(|s| s.to_string())
            .collect();

        let kv = cx.kv("YUMI")?;

        let mut proxy_kv_str = kv
            .get("proxy_kv")
            .text()
            .await?
            .unwrap_or_default();

        let mut rand_buf = [0u8; 1];

        getrandom::getrandom(&mut rand_buf)
            .expect("failed generating random number");

        if proxy_kv_str.is_empty() {
            let url =
                "https://raw.githubusercontent.com/jwfathoni17/xrayjwf/main/proxy.json";

            let req = Fetch::Url(Url::parse(url)?);

            let mut res = req.send().await?;

            if res.status_code() == 200 {
                proxy_kv_str = res.text().await?;

                kv.put("proxy_kv", &proxy_kv_str)?
                    .expiration_ttl(60 * 60 * 12)
                    .execute()
                    .await?;
            } else {
                return Err(Error::from(format!(
                    "error getting proxy kv: {}",
                    res.status_code()
                )));
            }
        }

        let proxy_kv: HashMap<String, Vec<String>> =
            serde_json::from_str(&proxy_kv_str)?;

        let kv_index =
            (rand_buf[0] as usize) % kvid_list.len();

        proxyip = kvid_list[kv_index].clone();

        let proxyip_index =
            (rand_buf[0] as usize) % proxy_kv[&proxyip].len();

        proxyip = proxy_kv[&proxyip][proxyip_index]
            .clone()
            .replace(':', "-");
    }

    let upgrade = req
        .headers()
        .get("Upgrade")?
        .unwrap_or_default();

    if upgrade == "websocket"
        && PROXYIP_PATTERN.is_match(&proxyip)
    {
        if let Some((addr, port_str)) =
            proxyip.split_once('-')
        {
            if let Ok(port) = port_str.parse() {
                cx.data.proxy_addr = addr.to_string();
                cx.data.proxy_port = port;
            }
        }

        let WebSocketPair { server, client } =
            WebSocketPair::new()?;

        server.accept()?;

        wasm_bindgen_futures::spawn_local(async move {
            let events = server.events().unwrap();

            if let Err(e) =
                ProxyStream::new(
                    cx.data,
                    &server,
                    events,
                )
                .process()
                .await
            {
                console_error!("[tunnel]: {}", e);
            }
        });

        Response::from_websocket(client)
    } else {
        Response::from_html("Xray Worker")
    }
}


// ============================================================
// CONFIG GENERATOR
// ============================================================

fn link(
    req: Request,
    cx: RouteContext<Config>,
) -> Result<Response> {
    let url = req.url()?;

    let wc = url
        .query_pairs()
        .find(|(key, _)| key == "wc")
        .map(|(_, value)| value.to_string())
        .unwrap_or_default();

    let custom = url
        .query_pairs()
        .find(|(key, _)| key == "custom")
        .map(|(_, value)| value.to_string())
        .unwrap_or_default();

    let (server, host) = build_target(&wc, &custom);

    let uuid = cx.data.uuid.to_string();

    let vless = format!(
        "vless://{uuid}@{server}:443\
?encryption=none\
&type=ws\
&host={host}\
&path=%2FID\
&security=tls\
&sni={host}\
#Changli-VLESS"
    );

    let clash = format!(
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
      path: /ID
      headers:
        Host: {host}"#,
        server = server,
        uuid = uuid,
        host = host,
    );

    let output = format!(
        "{vless}\n---CLASH---\n{clash}"
    );

    Response::from_body(
        ResponseBody::Body(output.into())
    )
}


// ============================================================
// TARGET LOGIC
// ============================================================

fn build_target(
    wc: &str,
    custom: &str,
) -> (String, String) {

    if wc.is_empty() || wc == "nonwc" {
        return (
            BASE_DOMAIN.to_string(),
            BASE_DOMAIN.to_string(),
        );
    }

    let value = if wc == "custom" {
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
        format!("{}.{}", value, BASE_DOMAIN);

    (
        value.to_string(),
        wildcard_host,
    )
}
