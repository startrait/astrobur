use crate::http::error::BurError;
use once_cell::sync::OnceCell;
use sqlx::postgres::{PgConnectOptions, PgPool, PgSslMode};
use sqlx::ConnectOptions;
use std::sync::Arc;

use crate::queue;
use crate::queue::{ClickCountJob, EngagementDetailJob};
use apalis::prelude::*;
use apalis_redis::{ConnectionManager, RedisStorage};
use serde::{Deserialize, Serialize};
use std::time::Duration;

pub struct AppState {
    pub db: Arc<PgPool>,
    pub job: Jobs,
}

pub struct Jobs {
    pub engagement_job: RedisStorage<EngagementDetailJob>,
    pub click_count_job: RedisStorage<ClickCountJob>,
}

impl AppState {
    pub async fn new() -> Result<Self, BurError> {
        let pg_pool = Arc::new(connect_postgres_db().await);

        let connection = queue::init_apalis(pg_pool.clone()).await?;
        let engagement_storage = queue::engagement_detail_worker(connection.clone()).await;
        let click_count_storage = queue::click_count_worker(connection.clone()).await;

        Ok(Self {
            db: pg_pool,
            job: Jobs {
                engagement_job: engagement_storage,
                click_count_job: click_count_storage,
            },
        })
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
