use crate::app_state::AppState;
use crate::http::error::BurError;
use crate::http::request_payload::UrlGenerationRequest;
use crate::queue::{ClickCountJob, EngagementDetailJob};
use crate::service::app_service::{create_url, get_url_details_from_code,get_qr_svg};
use apalis::prelude::*;
use axum::body::Body;
use axum::extract::{Json, Path, Query, State};
use axum::http::{header, StatusCode};
use axum::response::{IntoResponse, Response};
use axum::{
    routing::{get, post},
    Router,
};
use crate::http::response::{UrlInfo};
use std::collections::HashMap;
use tracing::info;

use std::sync::Arc;

pub fn router(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/api/generate", post(url_generator))
        .route("/api/url/{code}/info", get(url_info))
        .route("/{code}", get(url_handler))
        .with_state(state)
}

async fn url_handler(
    State(state): State<Arc<AppState>>,
    Path(path): Path<String>,
    Query(params): Query<HashMap<String, String>>,
) -> Result<Response, BurError> {

    let qr_scanned = params
        .get("qr_scanned")
        .map_or(false, |scanned| scanned == "true");

    info!("scanning");

    let url_detail = get_url_details_from_code(state.db.as_ref(), &path).await?;
    let mut destination = format!("{}", &url_detail.destination);

    if let Some(query_parameters) =
        serde_json::from_value::<Option<HashMap<String, String>>>(url_detail.query_parameters)?
    {
        let params = query_parameters
            .into_iter()
            .map(|(k, v)| format!("{}={}", k, v))
            .collect::<Vec<String>>()
            .join("&");

        destination = destination + "?" + &params;
    }

    let response = Response::builder()
        .status(StatusCode::PERMANENT_REDIRECT)
        .header(header::LOCATION, &destination)
        .body(Body::empty())
        .unwrap();

    let mut tracker = state.job.engagement_job.clone();
    match tracker
        .push(EngagementDetailJob {
            code: path.clone(),
            country: None,
            device: None,
            headers: None,
            ip: None,
        })
        .await
    {
        Ok(_) => {}
        Err(e) => println!("Failed to push engagement job {}", e),
    };

    let mut counter = state.job.click_count_job.clone();
    match counter.push(ClickCountJob { id: 1, qr_scanned }).await {
        Ok(_) => {}
        Err(e) => println!("Failed to push click count job {}", e),
    };

    Ok(response)
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

async fn url_info(
    State(state): State<Arc<AppState>>,
    Path(path): Path<String>
) -> Result<UrlInfo,BurError> {

    let url = get_url_details_from_code(state.db.as_ref(),&path).await?;
    let mut url_info: UrlInfo = url.into();

    let svg_xml = get_qr_svg(&path)?;
    url_info.qr_svg = Some(svg_xml);
    Ok(url_info)
}
