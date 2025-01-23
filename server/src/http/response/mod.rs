use crate::database::models::{Url};
use serde::Serialize;
use axum::response::{Response,IntoResponse};
use axum::http::StatusCode;
use axum::extract::Json;

#[derive(Serialize)]
pub struct UrlInfo {
    pub code: String,
    pub destination: String,
    pub track_qr_scans: bool,
    pub query_parameters: serde_json::Value,
    pub active: bool,
    pub qr_svg: Option<String>
}

impl IntoResponse for UrlInfo {

    fn into_response(self) -> Response {
        (StatusCode::OK,Json(&self)).into_response()
    }

}



