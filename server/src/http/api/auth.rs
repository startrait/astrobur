use crate::app::AppState;
use crate::error::BurError;
use crate::http::api::request::auth::{TokenValidationRequest, UserLoginRequest};
use crate::service::check_if_exists;
use crate::service::jwt::{generate_jwt, validate_jwt};
use crate::service::{auth, user};
use axum::extract::{Json, State};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::{
    routing::{get, post},
    Router,
};
use bcrypt::{hash, DEFAULT_COST};
use std::sync::Arc;

pub fn router(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/auth/user/login", post(user_login))
        .route("/validate-token", post(validate_token))
        .with_state(state)
}

async fn validate_token(Json(body): Json<TokenValidationRequest>) -> Result<Response, BurError> {
    let _validation = validate_jwt(&body.token)?;

    Ok((StatusCode::OK, "valid token").into_response())
}

async fn user_login(
    State(state): State<Arc<AppState>>,
    Json(mut body): Json<UserLoginRequest>,
) -> Result<Response, BurError> {
    let id = check_if_exists(state.db.as_ref(), "users", "email", &body.email).await?;
    let user = user::get_user(state, id).await?;
    if !bcrypt::verify(body.password, &user.password)? {
        return Err(BurError::InvalidCredential);
    }
    let response = auth::authenticate_user(&user).await?;

    Ok((StatusCode::OK, Json(response)).into_response())
}
