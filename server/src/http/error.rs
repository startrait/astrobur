use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use bcrypt::BcryptError;
use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum BurError {
    #[error("something went wrong")]
    DefaultError,
    #[error("app state failed ")]
    AppState(ErrorResponse),
    #[error("non-existent error")]
    DBError(ErrorResponse),
    #[error("sql error: {0}")]
    SQLxError(#[from] sqlx::Error),
    #[error("bcrypt error: {0}")]
    BcryptError(#[from] bcrypt::BcryptError),
    #[error("custom-error")]
    CustomError(ErrorResponse),
    #[error("chrono-error: {0}")]
    ChronoError(#[from] chrono::ParseError),
    #[error("serde_json-error: {0}")]
    SerdeJsonError(#[from] serde_json::Error),
}

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub error_code: u32,
    pub reason: String,
}

impl IntoResponse for BurError {
    fn into_response(self) -> Response {
        let error_response: (StatusCode, ErrorResponse) = match self {
            BurError::DefaultError => (
                StatusCode::INTERNAL_SERVER_ERROR,
                ErrorResponse {
                    error_code: 500,
                    reason: "Something went wrong".to_string(),
                },
            ),
            BurError::AppState(app_state_err) => (StatusCode::INTERNAL_SERVER_ERROR, app_state_err),

            BurError::SQLxError(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                ErrorResponse {
                    error_code: 521,
                    reason: format!("{}", err),
                },
            ),
            BurError::CustomError(err) => (StatusCode::INTERNAL_SERVER_ERROR, err),

            _ => (
                StatusCode::INTERNAL_SERVER_ERROR,
                ErrorResponse {
                    error_code: 500,
                    reason: "Something went wrong".to_string(),
                },
            ),
        };
        (error_response.0, Json(error_response.1)).into_response()
    }
}
