use crate::error::BurError;
use once_cell::sync::OnceCell;
use sqlx::postgres::{PgConnectOptions, PgPool, PgSslMode};
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
