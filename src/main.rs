// Imports - crates
use chrono::prelude::*;
// Imports - lib
use rust_async_1::*;

async fn run_app() {
    println!("Running application...");
    println!("Sleep duration in milliseconds {:?}", SLEEP_DURATION);
    println!();

    println!("Getting async task result...");

    let mut start_time = Local::now();

    let async_result = run_async_task().await;

    let mut end_time = Local::now();
    let mut duration = end_time
        .signed_duration_since(start_time)
        .num_milliseconds();

    println!("Async task result: {}", async_result);
    println!("Time taken: {} ms", duration);
    println!();

    println!("Getting self-made future task result...");

    start_time = Local::now();

    let self_made_result = self_made_future_task().await;

    end_time = Local::now();
    duration = end_time
        .signed_duration_since(start_time)
        .num_milliseconds();

    println!("Self-made future task result: {}", self_made_result);
    println!("Time taken: {} ms", duration);
}

#[tokio::main]
async fn main() {
    println!("START");
    println!();

    run_app().await;

    println!();
    println!("END");
}
