use server::http::http_server;
// use tracing_subscriber::fmt;
#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_writer(std::io::stdout)
        .json()
        .compact()
        .init();

    http_server::start().await.unwrap();
}
