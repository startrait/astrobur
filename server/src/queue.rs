use crate::http::error::BurError;
use apalis::prelude::*;
use apalis_redis::{ConnectionManager, RedisStorage};
use once_cell::sync::OnceCell;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use sqlx::Row;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;

static PG_CONNECTION: OnceCell<Arc<PgPool>> = OnceCell::new();

#[derive(Serialize, Deserialize)]
pub struct EngagementDetailJob {
    pub code: String,
    pub device: Option<String>,
    pub country: Option<[u8; 2]>,
    pub ip: Option<String>,
    pub headers: Option<HashMap<String, String>>,
}

#[derive(Serialize, Deserialize)]
pub struct ClickCountJob {
    pub id: i32,
    pub code: String,
}

// does need a cleaner and simpler interface to be able to swiftly switch between
// multiple storage, ideally like the QUEUE_CONNECTION on laravel

pub async fn init_apalis(pg_pool: Arc<PgPool>) -> Result<ConnectionManager, BurError> {
    PG_CONNECTION.set(pg_pool).unwrap();
    let redis_url = std::env::var("REDIS_URL").expect("Missing env variable REDIS_URL");
    let conn = apalis_redis::connect(redis_url)
        .await
        .expect("Could not initialize apalis redis connection");
    Ok(conn)
}

pub async fn consume_click_count(cc: ClickCountJob) {

    if let Some(db) = PG_CONNECTION.get() {

        let _ = sqlx::query("
        UPDATE url_trackings
        SET click_count = url_trackings.click_count + 1 
        WHERE url_id = $1
            ")
            .bind(&cc.id)
            .execute(db.as_ref())
            .await
            .unwrap();

        println!("incremented in db successfully");
    }
}

pub async fn click_count_worker(connection: ConnectionManager) -> RedisStorage<ClickCountJob> {
    let storage: RedisStorage<ClickCountJob> = RedisStorage::new(connection);
    let worker_storage = storage.clone();

    tokio::task::spawn(async move {
        WorkerBuilder::new("queue_click_count")
            .concurrency(8)
            .backend(worker_storage)
            .build_fn(consume_click_count)
            .run()
            .await;
    });

    storage
}

async fn consume_engagement(detail: EngagementDetailJob) {

    if let Some(db) = PG_CONNECTION.get() {

        let result = sqlx::query(
            "
        SELECT destination,organization_id 
        FROM urls
        WHERE active = true 
        AND code = $1
        ",
        )
        .bind(&detail.code)
        .fetch_optional(db.as_ref())
        .await
        .unwrap();

        match result {
            Some(row) => {
                let destination: String = row.get("destination");
                println!("THIS IS DESTINATION: {}", destination);
            }
            None => println!("None data"),
        };

    }

    println!("IM BEING CONSUMEDDDD AAAAAHHHHHHH");
}

pub async fn engagement_detail_worker(
    conn: ConnectionManager,
) -> RedisStorage<EngagementDetailJob> {
    let storage: RedisStorage<EngagementDetailJob> = RedisStorage::new(conn);
    let worker_storage = storage.clone();

    tokio::task::spawn(async move {
        let worker = WorkerBuilder::new("queue_engagement_tracker")
            .concurrency(8)
            .backend(worker_storage)
            .build_fn(consume_engagement)
            .run()
            .await;
    });

    storage
}
