use deno_core::{JsRuntime, RuntimeOptions};
use deno_core::serde_json::Value;

pub fn init() -> JsRuntime {
    // init js runtime
    JsRuntime::new(RuntimeOptions::default())
}

pub fn run_script(runtime: &mut JsRuntime, script: String) {
    let res = runtime
        .execute_script(
            "<demo>",
            script
            // concat!(include_str!("js_hijack_before.js"), include_str!("../demo.js"), include_str!("js_hijack_after.js")),
        )
        .unwrap();
    let mut handle = runtime.handle_scope();
    let rtn = res.open(&mut handle);
    let obj = rtn.to_rust_string_lossy(&mut handle);
    let val: Value = deno_core::serde_json::from_str(&obj).unwrap();
    dbg!(val);
}