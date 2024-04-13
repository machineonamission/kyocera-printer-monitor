mod js;
mod http;
mod check_one_printer;

#[tokio::main]
async fn main() {
    let mut runtime = js::init();
    check_one_printer::check_printer("165.134.155.129", &mut runtime).await.unwrap();
}
