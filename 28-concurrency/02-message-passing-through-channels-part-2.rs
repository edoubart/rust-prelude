/*
 * Message Passing Through Channels (Part 2)
 */
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    let (tx /*: Sender<{unknown}>*/, rx /*: Receiver<{unknown}*/) = mpsc::channel();

    thread::spawn(move || {
        let x: String = "some_value".to_string();
        println!("Sending value {x}");
        //thread::sleep(Duration::from_secs(3));
        tx.send(x).unwrap();
    });

    //let recv_val: String = rx.recv().unwrap();
    //println!("I am blocked.");

    let mut received_status: bool = false;
    while received_status != true {
        match rx.try_recv() {
            Ok(received_value/*: String*/) => {
                println!("Received value is: {received_value}");
                received_status = true;
            }
            Err(_) => println!("I am doing some other stuff"),
        }
    }
}
