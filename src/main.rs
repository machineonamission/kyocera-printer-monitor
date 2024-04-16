mod js;
mod http;
mod check_one_printer;
use std::sync::Arc;
use tokio::sync::Mutex;

#[tokio::main]
async fn main() {
    let async_runtime = Arc::new(Mutex::new(js::init()));
    check_one_printer::check_printer("165.134.87.74", async_runtime.clone()).await.unwrap();
}
