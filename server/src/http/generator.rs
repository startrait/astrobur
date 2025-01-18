use crate::app_state::AppState;
use crate::http::error::BurError;
use crate::http::request_payload::UrlGenerationRequest;
use crate::service::app_service::create_url;
use axum::extract::{Json, State};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::{
    routing::{get, post},
    Router,
};
use std::sync::Arc;

pub fn router(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/generate", post(url_generator))
        .with_state(state)
}

async fn url_generator(
    State(state): State<Arc<AppState>>,
    Json(body): Json<UrlGenerationRequest>,
) -> Result<Response, BurError> {
    let stringified = serde_json::to_string(&body).unwrap();
    create_url(state, body.try_into()?).await?;
    println!("{}", stringified);

    Ok((StatusCode::OK, "i am generator").into_response())
}
