use crate::error::{BurError, ErrorResponse};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use tracing::error;

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
            BurError::AppState(ref app_state_err) => {
                (StatusCode::INTERNAL_SERVER_ERROR, app_state_err.clone())
            }

            BurError::SQLxError(ref err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                ErrorResponse {
                    error_code: 521,
                    reason: format!("{}", err),
                },
            ),
            BurError::CustomError(ref err) => (StatusCode::INTERNAL_SERVER_ERROR, err.clone()),
            BurError::JsonWebTokenError(ref err) => (
                StatusCode::UNAUTHORIZED,
                ErrorResponse {
                    error_code: 401,
                    reason: format!("{}", err),
                },
            ),
            BurError::EntityNotFound => (
                StatusCode::BAD_REQUEST,
                ErrorResponse {
                    error_code: 400,
                    reason: "requested entity not found".to_string(),
                },
            ),

            _ => (
                StatusCode::INTERNAL_SERVER_ERROR,
                ErrorResponse {
                    error_code: 500,
                    reason: "Something went wrong".to_string(),
                },
            ),
        };
        error!("{}: {}", self, &error_response.1.reason);
        (error_response.0, Json(error_response.1)).into_response()
    }
}
