use crate::http::error::BurError;
use once_cell::sync::OnceCell;
use sqlx::postgres::{PgConnectOptions, PgPool, PgSslMode};
use sqlx::ConnectOptions;
use std::sync::Arc;

pub struct AppState {
    pub db: Arc<PgPool>,
}

pub static APP_SECRET: OnceCell<&'static str> = OnceCell::new();

impl AppState {
    pub async fn new() -> Result<Self, BurError> {
        let pg_pool = Arc::new(connect_postgres_db().await);
        APP_SECRET.set("GETFROMENV").unwrap();
        Ok(Self { db: pg_pool })
    }
}

async fn connect_postgres_db() -> PgPool {
    let opts = PgConnectOptions::new()
        .host("localhost")
        .port(5432)
        .username("siri")
        .password("siri")
        .database("astrobur")
        .ssl_mode(PgSslMode::Disable);
    // .connect()
    // .await
    // .unwrap();

    // opts =  opts.log_statements()
    let connection = PgPool::connect_with(opts).await.unwrap();
    connection
}
