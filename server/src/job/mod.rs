pub mod url;

use apalis_redis::RedisStorage;

pub struct Job {
    pub engagement_detail: RedisStorage<url::EngagementDetailJob>,
    pub click_count: RedisStorage<url::ClickCountJob>,
}
