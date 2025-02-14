pub mod api;
pub mod error;

use crate::app::AppState;
use crate::error::BurError;
use axum::Router;
use std::sync::Arc;
use tokio::net::TcpListener;

pub async fn start() -> Result<(), BurError> {
    let app_state = Arc::new(AppState::new().await.unwrap());
    let router = Router::new().merge(api::router(app_state));

    let host = std::env::var("HOST").expect("Expected environment variable HOST");
    let port = std::env::var("PORT").expect("Expected environment variable PORT");

    let listener = TcpListener::bind(format!("{}:{}", host, port))
        .await
        .unwrap();

    axum::serve(listener, router).await.unwrap();
    Ok(())
}
