use crate::app_state::AppState;
use crate::database::models::User;
use crate::http::error::BurError;
use crate::http::request_payload::UserCreationRequest;
use crate::service::app_service::create_user;
use axum::extract::{Json, State};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::{
    routing::{get, post},
    Router,
};
use bcrypt::{hash, DEFAULT_COST};
use serde_json::json;
use std::sync::Arc;

pub fn router(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/authetication", get(get_authentication))
        .route("/create-user", post(http_create_user))
        .with_state(state)
}

async fn get_authentication() -> Response {
    (StatusCode::OK, "/authetication").into_response()
}

async fn http_create_user(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<UserCreationRequest>,
) -> Result<Response, BurError> {
    let mut user: User = payload.into();
    user.password = hash(user.password, DEFAULT_COST)?;

    create_user(user, state.clone()).await?;

    Ok((
        StatusCode::OK,
        Json(json!({
            "message": "User created successfully"
        })),
    )
        .into_response())
}
