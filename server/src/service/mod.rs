pub mod app_service;
pub mod jwt_service;
use chrono::NaiveDateTime;

pub struct AuthenticatedUser {
    pub user_id: i32,
}
