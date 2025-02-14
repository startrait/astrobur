pub mod auth;
pub mod jwt;
pub mod url;
pub mod user;

use crate::error::{BurError, ErrorResponse};
use sqlx::postgres::PgPool;
use sqlx::Row;

pub async fn check_if_exists(
    db: &PgPool,
    table: &str,
    column: &str,
    value: &String,
) -> Result<i32, BurError> {
    let query = format!("SELECT id FROM {} WHERE {} = $1", table, column);

    let result = sqlx::query(&query).bind(value).fetch_optional(db).await?;

    match result {
        Some(row) => Ok(row.get::<i32, _>("id")),
        None => Err(BurError::EntityNotFound),
    }
}
