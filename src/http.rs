use crate::js;
use anyhow::Result;
use deno_core::JsRuntime;
use serde_json::Value;

pub fn fetch(url: &str) -> Result<String> {
    let res = ureq::get(url).call()?;
    Ok(res.into_string()?)
}

pub fn fetch_object(url: &str, runtime: &mut JsRuntime) -> Result<Value> {
    let script = fetch(url)?;
    let obj = js::cursed_js_to_object(runtime, script)?;
    Ok(obj)
}