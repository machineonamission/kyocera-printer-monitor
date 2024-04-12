mod js;

fn main() {
    let mut runtime = js::init();
    js::run_script(
        &mut runtime,
        concat!(include_str!("js_hijack_before.js"), include_str!("../demo.js"), include_str!("js_hijack_after.js")).to_string(),
    );
}
