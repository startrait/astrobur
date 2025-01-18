use crate::app_state::AppState;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::{routing::get, Router};
use std::sync::Arc;

pub fn router(state: Arc<AppState>) -> Router {
    Router::new().route("/analytics", get(get_link_analytics))
}

async fn get_link_analytics() -> Response {
    (StatusCode::OK, "hello").into_response()
}
