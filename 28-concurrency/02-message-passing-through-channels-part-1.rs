/*
 * Message Passing Through Channels (Part 1)
 */
use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx /*: Sender<{unknown}>*/, rx /*: Receiver<{unknown}*/) = mpsc::channel();

    //thread::spawn(move || {
    //    let mut i: String = "S".to_string();
    //    //let mut i: i32 = 5;
    //    println!("Sending value {i}");
    //    tx.send(i).unwrap();
    //    // error[E0382]: borrow of moved value: `i`
    //    //println!("Val is: {i}");
    //});

    //let received_val: String = rx.recv().unwrap();
    //println!("Received {received_val}");

    for i /*: i32*/ in 0..10 {
        let tx_clone = tx.clone();
        thread::spawn(move || {
            //let mut i: String = "S".to_string();
            println!("Sending value {i}");
            tx_clone.send(i).unwrap();
            //println!("Val is: {i}");
        });
    }

    drop(tx);

    //let received_val: i32 = rx.recv().unwrap();
    //println!("Received {received_val}");

    //let received_val: i32 = rx.recv().unwrap();
    //println!("Received {received_val}");
    
    for message in rx {
        println!("Received {message}");
    }
}
