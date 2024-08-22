use std::rc::Rc;

use anyhow::{anyhow, Result};
use kg_js::JsEngine;
use tokio::sync::Mutex;

use crate::js;

pub async fn fetch(host: &str, path: &str) -> Result<String> {
    Ok(reqwest::Client::builder()
        // MOST printers FORCE https but none have a valid cert so bleh
        .danger_accept_invalid_certs(true)
        .build()?
        .get(reqwest::Url::parse(host)?.join(path)?)
        // without these 2 headers the request errors "internal server error" for some reason
        .header(reqwest::header::COOKIE, "rtl=0")
        .header(reqwest::header::REFERER, host)
        .send()
        .await?
        .text()
        .await?)
}

pub async fn get_right_host(ip: &str) -> Result<String> {
    // all of the printers ive tested EXCEPT ONE forcibly redirect to https
    // the stubborn non-https one ONLY does http, will not respond to https
    // make a head request to the printer to figure out what it wants

    // i cant just let it redirect on its own cause then i lose some headers which are required
    let resp = reqwest::Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .build()?
        .get(format!("http://{ip}"))
        .send()
        .await?;
    match resp.status().as_u16() {
        200 => {
            // no redirect, http is fine
            Ok(format!("http://{ip}"))
        }
        307 => {
            // redirect, upgrade to https
            Ok(format!("https://{ip}"))
        }
        e => {
            // something else, probably error?
            Err(anyhow!(
                "Head request to {ip} failed with code {e}. {resp:?}"
            ))
        }
    }
}

pub async fn fetch_object(
    host: &str,
    path: &str,
    runtime: Rc<Mutex<JsEngine>>,
) -> Result<js::Object> {
    let script = fetch(host, path).await?;
    let obj = js::cjto_locking(runtime, script).await?;
    Ok(obj)
}
