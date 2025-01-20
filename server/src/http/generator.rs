use crate::app_state::{AppState};
use crate::queue::{EngagementDetailJob,ClickCountJob};
use crate::http::error::BurError;
use crate::http::request_payload::UrlGenerationRequest;
use crate::service::app_service::create_url;
use apalis::prelude::*;
use axum::extract::{Json, State,Path};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::{
    routing::{get, post},
    Router,
};

use std::sync::Arc;

pub fn router(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/api/generate", post(url_generator))
        .route("/{code}", get(url_handler))
        .with_state(state)
}

async fn url_handler(
    State(state): State<Arc<AppState>>,
    Path(path): Path<String>
    ) -> Response {

    let mut tracker = state.job.engagement_job.clone();
    tracker
        .push(EngagementDetailJob {
            code: path.clone(),
            country: None,
            device: None,
            headers: None,
            ip: None
        })
        .await
        .unwrap();
    let mut counter = state.job.click_count_job.clone();
    counter.push(ClickCountJob {
        id: 1,
        code: path
    })
    .await
    .unwrap();

    (StatusCode::OK, "i am url handler").into_response()
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
