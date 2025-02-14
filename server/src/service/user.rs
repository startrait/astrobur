use crate::app::AppState;
use crate::database::models::user::User;
use crate::error::BurError;
use crate::service::check_if_exists;
use sqlx::postgres::PgPool;
use sqlx::Row;
use std::sync::Arc;
use tracing::event;
use tracing::Level;

pub async fn create_user(user: &User, state: Arc<AppState>) -> Result<i32, BurError> {
    let db: Arc<PgPool> = state.db.clone();

    check_if_exists(db.as_ref(), "users", "email", &user.email).await?;

    let mut tx = db.as_ref().begin().await?;

    let user_id: i32 =
        sqlx::query("INSERT INTO users(name,email,password) VALUES($1,$2,$3) RETURNING ID")
            .bind(&user.name)
            .bind(&user.email)
            .bind(&user.password)
            .fetch_one(&mut *tx)
            .await?
            .get("id");

    let _ = sqlx::query("INSERT INTO organizations(name,root_user) VALUES($1,$2) RETURNING ID")
        .bind(&user.name)
        .bind(&user_id)
        .fetch_one(&mut *tx)
        .await?;

    tx.commit().await?;
    event!(Level::INFO, message = "User created.", user = &user.email);

    Ok(user_id)
}

pub async fn get_user(state: Arc<AppState>, id: i32) -> Result<User, BurError> {
    let row = sqlx::query("SELECT id, name, email, password FROM users WHERE id = $1")
        .bind(&id)
        .fetch_one(state.db.as_ref())
        .await
        .unwrap();

    Ok(User {
        id: Some(row.get("id")),
        name: row.get("name"),
        email: row.get("email"),
        password: row.get("password"),
    })
}
