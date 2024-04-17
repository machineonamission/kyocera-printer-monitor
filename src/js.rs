use std::sync::Arc;

use anyhow::{anyhow, Result};
use deno_core::{JsRuntime, RuntimeOptions};
use serde_json::{Map, Value};
use tokio::sync::Mutex;

pub(crate) type Object = Map<String, Value>;

pub fn init() -> JsRuntime {
    // init js runtime
    JsRuntime::new(RuntimeOptions::default())
}

pub fn cursed_js_to_object(runtime: &mut JsRuntime, script: String) -> Result<Object> {
    let res = runtime.execute_script(
        "<demo>",
        format!(
            "{}\n{}\n{}",
            include_str!("js_hijack_before.js"),
            script,
            include_str!("js_hijack_after.js")
        ),
    )?;
    let mut handle = runtime.handle_scope();
    let rtn = res.open(&mut handle);
    let obj = rtn.to_rust_string_lossy(&mut handle);
    let val: Value = serde_json::from_str(&obj)?;
    if let Value::Object(map) = val {
        Ok(map)
    } else {
        Err(anyhow!("JS did not return an object, but instead: {val:?}"))
    }
}

pub async fn CJTO_locking(runtime: Arc<Mutex<JsRuntime>>, script: String) -> Result<Object> {
    // grabs the lock, runs the function, and then releases the lock
    let mut runtime = runtime.lock().await;
    cursed_js_to_object(&mut runtime, script)
}
