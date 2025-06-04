// Imports - crates
use std::future::Future;
use tokio::time::{sleep, Duration};

pub const SLEEP_DURATION: Duration = Duration::from_millis(100);

async fn async_task() -> u32 {
    sleep(SLEEP_DURATION).await;
    56
}

pub async fn run_async_task() -> u32 {
    let result = async_task().await;

    result
}

pub fn self_made_future_task() -> impl Future<Output = u32> {
    async {
        sleep(SLEEP_DURATION).await;
        63
    }
}
