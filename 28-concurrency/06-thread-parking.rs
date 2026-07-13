/*
 * Thread Park
 */
use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};
use std::time::Duration;

fn main() {
    let data: Arc<Mutex<i32>> = Arc::new(Mutex::new(5));
    let data_clone = data.clone();

    let thread_1: JoinHandle<()> = thread::spawn(move || {
        println!("Thread 1: I am doing some work");
        println!("Thread 1: I am doing some more work");
        //thread::sleep(Duration::from_secs(2)); // -> Use Thread Park.

        println!("Thread 1: Parked");
        //thread::park();
        thread::park_timeout(Duration::from_secs(4));

        println!("Thread 1: Printing the updated data");
        println!("Thread 1: Data: {:?}", *data.lock().unwrap());
    });
    
    let thread_2: JoinHandle<()> = thread::spawn(move || {
        println!("Thread 2: I am working on updating the data");
        thread::sleep(Duration::from_secs(1));
        *data_clone.lock().unwrap() = 10; // error[E0382]: use of moved value: `data` -> data_clone = data.clone()
        println!("Thread 2: Data updated completed");
    });

    thread_2.join();
    //thread_1.thread().unpark();
    thread_1.join();
}
