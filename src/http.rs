use crate::js;
use anyhow::Result;
use deno_core::JsRuntime;
use serde_json::Value;
use std::sync::Arc;
use tokio::sync::Mutex;

pub async fn fetch(host:&str, path:&str) -> Result<String> {
    Ok(reqwest::Client::builder()
        // printer forces https but doesn't have a valid cert so fuck you
        .danger_accept_invalid_certs(true)
        .build()?
        .get(reqwest::Url::parse(host)?.join(path)?)
        .header(reqwest::header::COOKIE, "rtl=0")
        .header(reqwest::header::REFERER, host)
        .send()
        .await?
        .text()
        .await?)
}

pub async fn fetch_object(host: &str, path:&str, runtime: Arc<Mutex<JsRuntime>>) -> Result<Value> {
    let script = fetch(host, path).await?;
    let obj = js::CJTO_locking(runtime, script).await?;
    Ok(obj)
}