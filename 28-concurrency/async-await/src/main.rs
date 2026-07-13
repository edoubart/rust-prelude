/*
 * Async / Await
 */
use std::future::Future;

async fn printing() {
    println!("I am an async function.")
}

#[tokio::main] // Attribute Macro
async fn main() {
    //let x/*: impl Future<Output = ()>*/ = printing();
    //let x/*()*/ = printing().await;
    let x/*: impl Future<Output = ()>*/ = printing();
    println!("The future has not been polled yet.");
    drop(x);

    //x.await;
}
