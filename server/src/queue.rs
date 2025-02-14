use crate::error::BurError;
use crate::job::{
    url::{consume_click_count_init, consume_engagement_init},
    Job,
};

pub async fn queue_init() -> Result<Job, BurError> {
    let redis_url = std::env::var("REDIS_URL").expect(" Expected REDIS_URL environment variable.");
    let conn = apalis_redis::connect(redis_url)
        .await
        .expect("Failed to initialize queue connection with redis.");

    Ok(Job {
        engagement_detail: consume_engagement_init(conn.clone()).await,
        click_count: consume_click_count_init(conn.clone()).await,
    })
}

#[macro_export]
macro_rules! register_job {
    ($job_fn_name:ident,$job_t:ident,$job_fn:ident,$conc:expr) => {
        pub async fn $job_fn_name(connection: ConnectionManager) -> RedisStorage<$job_t> {
            let storage: RedisStorage<$job_t> = RedisStorage::new(connection);
            let worker_storage = storage.clone();

            tokio::task::spawn(async move {
                WorkerBuilder::new(stringify!($job_fn))
                    .concurrency($conc)
                    .backend(worker_storage)
                    .build_fn($job_fn)
                    .run()
                    .await;
            });

            storage
        }
    };
}
