use crate::database::models::user::User;
use crate::error::BurError;
use crate::http::api::response::auth::AuthenticatedUserResponse;
use crate::service::check_if_exists;
use crate::service::jwt::generate_jwt;
use chrono::Local;
use std::ops::Add;
use std::time::Duration;

pub async fn authenticate_user(user: &User) -> Result<AuthenticatedUserResponse, BurError> {
    let exp = Local::now().add(Duration::from_secs(86400)).timestamp() as usize;
    let token = generate_jwt(user, Some(exp))?;

    Ok(AuthenticatedUserResponse {
        access_token: token,
        refresh_token: None,
    })
}
