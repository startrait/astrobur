mod auth;
pub mod request;
pub mod response;
mod url;

use crate::app::AppState;
use axum::Router;
use std::sync::Arc;

pub fn router(state: Arc<AppState>) -> axum::Router {
    Router::new()
        .merge(url::router(state.clone()))
        .merge(auth::router(state.clone()))
}
