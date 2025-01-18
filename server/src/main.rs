use server::http::http_server;
#[tokio::main]
async fn main() {
    http_server::start().await.unwrap();
}
