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

    let listener = TcpListener::bind("0.0.0.0:7777").await.unwrap();
    axum::serve(listener, router).await.unwrap();
    Ok(())
}
