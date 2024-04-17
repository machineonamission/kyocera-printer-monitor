use std::sync::Arc;

use tokio::sync::Mutex;

mod check_one_printer;
mod http;
mod js;
mod r#static;

#[tokio::main]
async fn main() {
    let async_runtime = Arc::new(Mutex::new(js::init()));
    check_one_printer::check_printer("  ", async_runtime.clone())
        .await
        .unwrap();
}
