use crate::app::PG_CONNECTION;
use crate::register_job;
use apalis::prelude::*;
use apalis_redis::{ConnectionManager, RedisStorage};
use serde::{Deserialize, Serialize};
use sqlx::Row;
use std::collections::HashMap;

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
    pub qr_scanned: bool,
    pub id: i32,
}

pub async fn consume_click_count(cc: ClickCountJob) {
    if let Some(db) = PG_CONNECTION.get() {
        let _ = sqlx::query(
            "
        UPDATE url_trackings
        SET total_click_count = url_trackings.total_click_count + 1,
            qr_scan_count = qr_scan_count + $1

        WHERE url_id = $2
            ",
        )
        .bind(*&cc.qr_scanned as i32)
        .bind(&cc.id)
        .execute(db.as_ref())
        .await
        .unwrap();

        println!("incremented in db successfully");
    }
}

register_job!(
    consume_click_count_init,
    ClickCountJob,
    consume_click_count,
    20
);

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

register_job!(
    consume_engagement_init,
    EngagementDetailJob,
    consume_engagement,
    20
);
