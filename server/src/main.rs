use server::http::start as start_http_server;
// use tracing_subscriber::fmt;
#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_writer(std::io::stdout)
        .json()
        .compact()
        .init();

    start_http_server().await.unwrap();
}
