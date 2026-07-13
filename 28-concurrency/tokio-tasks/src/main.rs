/*
 * Tokio Tasks
 */
use std::future::Future;
use std::time::Duration;
use tokio::task::JoinHandle;
use tokio::time::sleep;

async fn printing(i: i32) {
    sleep(Duration::from_secs(1)).await; // Concurrent execution (non-deterministic)
    println!("Task {i}");
}

#[tokio::main] // Concurrent execution (non-deterministic)
//#[tokio::main(flavor = "current_thread")] // Sequential execution
async fn main() {
    let mut handles: Vec<JoinHandle<()>> = vec![];

    for i in 0..3 {
        let handle: JoinHandle<()> = tokio::spawn(/*future:*/ async move {
            println!("Task {i}, printing, first time");
            printing(i).await;
            println!("Task {i}, printing, second time");
            printing(i).await;
            println!("Task {i}, printing, completed");
        });
        handles.push(handle);
    }

    for handle/*: JoinHandle<()>*/ in handles {
        handle.await.unwrap();
    }

    println!("All tasks are now completed.");
}
