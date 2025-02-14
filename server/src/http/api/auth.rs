use crate::app::AppState;
use crate::database::models::user::User;
use crate::error::BurError;
use crate::http::api::request::auth::TokenValidationRequest;
use crate::http::api::request::user::UserCreationRequest;

use crate::service::jwt_service::{generate_jwt, validate_jwt};
use crate::service::user::create_user;
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
        .route("/validate-token", post(validate_token))
        .with_state(state)
}

#[axum::debug_handler]
async fn validate_token(Json(payload): Json<TokenValidationRequest>) -> Result<Response, BurError> {
    let validation = validate_jwt(&payload.token)?;

    Ok((StatusCode::OK, "valid token").into_response())
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

    let user_id = create_user(&user, state.clone()).await?;
    user.id = Some(user_id);

    let token = generate_jwt(&user, None)?;

    Ok((
        StatusCode::OK,
        Json(json!({
            "message": "User created successfully",
            "access_token": token
        })),
    )
        .into_response())
}
