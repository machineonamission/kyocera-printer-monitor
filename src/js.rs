use std::rc::Rc;

use anyhow::{anyhow, Result};
use kg_js::JsEngine;
use serde_json::{Map, Value};
use tokio::sync::Mutex;

pub(crate) type Object = Map<String, Value>;

pub fn cursed_js_to_object(runtime: &mut JsEngine, script: String) -> Result<Object> {
    runtime.eval(&format!(
        "{}\n{}\n{}",
        include_str!("js_hijack_before.js"),
        script,
        include_str!("js_hijack_after.js")
    ))?;
    // get the object from the JS runtime, -1 is the last one i guess?
    let obj = runtime.get_string(-1);
    let val: Value = serde_json::from_str(&obj)?;
    if let Value::Object(map) = val {
        Ok(map)
    } else {
        Err(anyhow!("JS did not return an object, but instead: {val:?}"))
    }
}

pub async fn cjto_locking(runtime: Rc<Mutex<JsEngine>>, script: String) -> Result<Object> {
    // grabs the lock, runs the function, and then releases the lock
    let mut runtime = runtime.lock().await;
    cursed_js_to_object(&mut runtime, script)
}
