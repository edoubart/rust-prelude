/*
 * Async / Await
 */
use std::future::Future;

async fn printing() {
    println!("I am an async function.")
}

async fn main() {
    //let x/*: impl Future<Output = ()>*/ = printing();
    let x/*()*/ = printing().await;
}
