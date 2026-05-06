use std::time::{Duration, Instant};
use tokio::time::sleep;

#[tokio::main]
async fn main() {
    println!("lesson 1: sequential await");
    sequential_await().await;

    println!();
    println!("lesson 2: concurrent await with join!");
    concurrent_await().await;

    println!();
    println!("lesson 3: spawned tasks");
    spawned_tasks().await;
}

//async functions returns a future, which is a value that represents a computation that may not have completed yet.
async fn simulated_request(name: &str, delay_ms: u64) -> String { // slow IO. 
    
    println!("{name}: started");
    sleep(Duration::from_millis(delay_ms)).await;
    println!("{name}: finished");

    format!("{name} response")
}

// awaiting a future will yield control back to the executor, allowing other tasks to run while waiting for the result.
async fn sequential_await() {
    let start = Instant::now();

    let first = simulated_request("first", 1_000).await;
    let second = simulated_request("second", 1_000).await;

    println!("first result: {first}");
    println!("second result: {second}");
    println!("elapsed: {:?}", start.elapsed());
}

async fn concurrent_await() {
    let start = Instant::now();

    // tokio::join! allows you to run multiple async operations concurrently and wait for all of them to complete.
    let first = simulated_request("first", 1_000);
    let second = simulated_request("second", 1_000);

    // the join macro runs the 2 futures concurrently
    let (first, second) = tokio::join!(first, second);

    println!("first result: {first}");
    println!("second result: {second}");
    println!("elapsed: {:?}", start.elapsed());
}


// tokio::spawn allows you to run an async task in the background, and it returns a JoinHandle that you can await to get the result of the task.
async fn spawned_tasks() {
    let start = Instant::now();

    // starts a seperate tokio task for each simulated request, allowing them to run concurrently in the background.
    let first_handle = tokio::spawn(async {
        simulated_request("first spawned", 1_000).await
    });

    let second_handle = tokio::spawn(async {
        simulated_request("second spawned", 500).await
    });

    println!("both tasks are now running in the background");

    let first = first_handle.await.expect("first task panicked"); // awaiting the hanfle waits for the task to complete
    let second = second_handle.await.expect("second task panicked");

    println!("first result: {first}");
    println!("second result: {second}");
    println!("elapsed: {:?}", start.elapsed());
}
