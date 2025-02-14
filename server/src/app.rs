use crate::error::BurError;
use once_cell::sync::OnceCell;
use sqlx::postgres::{PgConnectOptions, PgPool, PgSslMode};
use sqlx::ConnectOptions;
use std::sync::Arc;

use crate::job::Job;
use crate::queue;

pub static PG_CONNECTION: OnceCell<Arc<PgPool>> = OnceCell::new();

pub struct AppState {
    pub db: Arc<PgPool>,
    pub job: Job,
}

impl AppState {
    pub async fn new() -> Result<Self, BurError> {
        let pg_pool = Arc::new(connect_postgres_db().await);
        let _ = PG_CONNECTION.set(pg_pool.clone());

        let job = queue::queue_init().await?;

        Ok(Self { db: pg_pool, job })
    }
}

async fn connect_postgres_db() -> PgPool {
    let host = std::env::var("DB_HOST").expect("Expected DB_HOST environment variable.");
    let port = std::env::var("DB_PORT")
        .expect("Expected DB_PORT environment variable.")
        .parse::<u16>()
        .expect("DB_PORT must be an 16 bit unsigned integer");
    let username =
        std::env::var("DB_USERNAME").expect("Expected DB_USERNAME environment variable.");
    let password =
        std::env::var("DB_PASSWORD").expect("Expected DB_PASSWORD environment variable.");
    let database =
        std::env::var("DB_DATABASE").expect("Expected DB_DATABASE environment variable.");

    let opts = PgConnectOptions::new()
        .host(&host)
        .port(port)
        .username(&username)
        .password(&password)
        .database(&database)
        .ssl_mode(PgSslMode::Disable);
    // .connect()
    // .await
    // .unwrap();

    //opts =  opts.log_statements();
    let connection = PgPool::connect_with(opts).await.unwrap();
    connection
}
