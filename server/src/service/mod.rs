pub mod jwt_service;
pub mod url;
pub mod user;


use crate::error::{BurError, ErrorResponse};
use sqlx::postgres::PgPool;

pub struct AuthenticatedUser {
    pub user_id: i32,
}

pub async fn check_if_exists(
    db: &PgPool,
    table: &str,
    column: &str,
    value: &String,
) -> Result<(), BurError> {
    let query = format!("SELECT 1 FROM {} WHERE {} = $1", table, column);

    let result = sqlx::query(&query).bind(&value).fetch_optional(db).await?;

    match result {
        Some(_) => Err(BurError::CustomError(ErrorResponse {
            error_code: 500,
            reason: format!("{} already exists", column),
        })),
        None => Ok(()),
    }
}
