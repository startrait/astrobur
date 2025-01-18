use crate::app_state::AppState;
use crate::http::analytics;
use crate::http::authenticator;
use crate::http::error::BurError;
use crate::http::generator;
use axum::Router;
use std::sync::Arc;
use tokio::net::TcpListener;

pub async fn start() -> Result<(), BurError> {
    let app_state = Arc::new(AppState::new().await.unwrap());
    let router = Router::new()
        .merge(analytics::router(app_state.clone()))
        .merge(generator::router(app_state.clone()))
        .merge(authenticator::router(app_state.clone()));

    let listener = TcpListener::bind("0.0.0.0:7777").await.unwrap();
    axum::serve(listener, router).await.unwrap();
    Ok(())
}
