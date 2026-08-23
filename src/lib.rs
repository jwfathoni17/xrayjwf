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
    .on_async("/:location/:server", selected_tunnel)
    .on_async("/:proxyip", tunnel)
    .on_async("/Benxx-Project/:proxyip", tunnel)
    .run(req, env)
    .await
}

fn web_ui(_: Request, _: RouteContext<Config>) -> Result<Response> {
    Response::from_html(WEB_UI)
}
async fn selected_tunnel(
    req: Request,
    mut cx: RouteContext<Config>,
) -> Result<Response> {

    let location = cx
        .param("location")
        .map(|x| x.to_uppercase())
        .unwrap_or_default();

    let server_index: usize = cx
        .param("server")
        .and_then(|x| x.parse::<usize>().ok())
        .unwrap_or(0);

    // Server path dimulai dari 1:
    // /ID/1 = index 0
    // /ID/2 = index 1
    // /ID/3 = index 2
    if server_index == 0 {
        return Response::error(
            "Invalid server number",
            400,
        );
    }

    let kv = cx.kv("YUMI")?;

    let mut proxy_kv_str = kv
        .get("proxy_kv")
        .text()
        .await?
        .unwrap_or_default();

    // Jika KV kosong, ambil proxy.json milik kita
    if proxy_kv_str.is_empty() {

        let url =
            "https://raw.githubusercontent.com/jwfathoni17/xrayjwf/main/proxy.json";

        let request =
            Fetch::Url(Url::parse(url)?);

        let mut response =
            request.send().await?;

        if response.status_code() != 200 {
            return Response::error(
                "Failed to load proxy configuration",
                502,
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
            Some(value) => value,
            None => {
                return Response::error(
                    "Location not found",
                    404,
                );
            }
        };

    // /ID/1 → index 0
    let index = server_index - 1;

    let selected =
        match servers.get(index) {
            Some(value) => value.clone(),
            None => {
                return Response::error(
                    "Server not found",
                    404,
                );
            }
        };

    // IP:PORT → IP-PORT
    let proxyip =
        selected.replace(':', "-");

    let upgrade =
        req.headers()
            .get("Upgrade")?
            .unwrap_or_default();

    if upgrade != "websocket" {
        return Response::from_html(
            format!(
                "Selected server: {}/{}",
                location,
                server_index
            )
        );
    }

    if !PROXYIP_PATTERN.is_match(&proxyip) {
        return Response::error(
            "Invalid proxy server",
            400,
        );
    }

    if let Some((addr, port_str)) =
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
    } = WebSocketPair::new()?;

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

    let location = url
        .query_pairs()
        .find(|(key, _)| key == "location")
        .map(|(_, value)| value.to_uppercase())
        .unwrap_or_else(|| "ID".to_string());

    let server = url
        .query_pairs()
        .find(|(key, _)| key == "server")
        .map(|(_, value)| value.to_string())
        .unwrap_or_else(|| "1".to_string());

    let path =
        format!("/{}/{}", location, server);

    let host =
        cx.data.host.to_string();

    let uuid =
        cx.data.uuid.to_string();


    // ==========================
    // VLESS
    // ==========================

    let vless =
        format!(
            "vless://{uuid}@{host}:443?encryption=none&type=ws&host={host}&path={}&security=tls&sni={host}#Changli-VLESS",
            urlencoding::encode(&path)
        );


    // ==========================
    // CLASH
    // ==========================

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
      path: {path}
      headers:
        Host: {host}"#,
            host = host,
            uuid = uuid,
            path = path,
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
