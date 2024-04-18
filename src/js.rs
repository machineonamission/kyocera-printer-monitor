use std::sync::Arc;

use anyhow::{anyhow, Result};
use serde_json::{Map, Value};
use tokio::sync::Mutex;

pub(crate) type Object = Map<String, Value>;

pub struct JsRuntime<'a> {
    platform: v8::SharedRef<v8::Platform>,
    isolate: v8::OwnedIsolate,
    handle_scope: v8::HandleScope<'a, ()>,
    context: v8::Local<'a, v8::Context>,
    context_scope: v8::ContextScope<'a, v8::HandleScope<'a>>,
}
impl<'a> JsRuntime<'_> {
    pub fn init(&mut self) {
        let platform = v8::new_default_platform(0, false).make_shared();
        v8::V8::initialize_platform(platform);
        v8::V8::initialize();

        // Create a new Isolate and make it the current one.
        self.isolate = v8::Isolate::new(v8::CreateParams::default());

        // Create a stack-allocated handle scope.
        self.handle_scope = v8::HandleScope::new(&mut self.isolate);

        // Create a new context.
        self.context = v8::Context::new(&mut self.handle_scope);

        // Enter the context for compiling and running the hello world script.
        self.context_scope = v8::ContextScope::new(&mut self.handle_scope, self.context);
    }
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
