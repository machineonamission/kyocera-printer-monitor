use deno_core::{anyhow, JsRuntime, RuntimeOptions};
use serde_json::Value;
use anyhow::Result;

pub fn init() -> JsRuntime {
    // init js runtime
    JsRuntime::new(RuntimeOptions::default())
}

pub fn cursed_js_to_object(runtime: &mut JsRuntime, script: String) -> Result<Value> {
    let res = runtime
        .execute_script(
            "<demo>",
            format!("{}{}{}", include_str!("js_hijack_before.js"), script, include_str!("js_hijack_after.js"))
        )?;
    let mut handle = runtime.handle_scope();
    let rtn = res.open(&mut handle);
    let obj = rtn.to_rust_string_lossy(&mut handle);
    let val: Value = serde_json::from_str(&obj)?;
    Ok(val)
}