use std::sync::Arc;

use anyhow::{anyhow, Result};
use serde_json::{Map, Value};
use tokio::sync::Mutex;

pub(crate) type Object = Map<String, Value>;

pub struct JsRuntime {
    platform: v8::SharedRef<v8::Platform>,
    //TODO
}

pub fn init() {
    let platform = v8::new_default_platform(0, false).make_shared();
    v8::V8::initialize_platform(platform);
    v8::V8::initialize();

    // Create a new Isolate and make it the current one.
    let isolate = &mut v8::Isolate::new(v8::CreateParams::default());

    // Create a stack-allocated handle scope.
    let handle_scope = &mut v8::HandleScope::new(isolate);

    // Create a new context.
    let context = v8::Context::new(handle_scope);

    // Enter the context for compiling and running the hello world script.
    v8::ContextScope::new(handle_scope, context)
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
