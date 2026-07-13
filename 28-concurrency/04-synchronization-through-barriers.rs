/*
 * Synchronization Through Barriers
 */
use std::sync::{Arc, Barrier, Mutex};
use std::thread;
use thread::JoinHandle;

fn main() {
    let mut threads_vec: Vec<JoinHandle<()>> = Vec::new();
    let tasks: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(vec![]));
    let barrier: Arc<Barrier> = Arc::new(Barrier::new(5));

    for i in 0..5 {
        let tasks: Arc<Mutex<Vec<String>>> = tasks.clone();
        let barrier: Arc<Barrier> = barrier.clone();
        let handle: JoinHandle<()> = thread::spawn(move || {
            // Task 1
            tasks // Arc<Mutex<Vec<String>>>
                .lock() // Result<MutexGuard<Vec<String>>>
                .unwrap() // MutexGuard<Vec<String>>
                .push(format!("Thread {i}, Completed its part on Task 1"));

            barrier.wait(); // Block the calling thread

            // Task 2
            tasks // Arc<Mutex<Vec<String>>>
                .lock() // Result<MutexGuard<Vec<String>>>
                .unwrap() // MutexGuard<Vec<String>>
                .push(format!("Thread {i}, Completed its part on Task 2"));
        });
        threads_vec.push(handle);
    }

    for handle /*: JoinHandle<()>*/ in threads_vec {
        handle.join().unwrap();
    }

    let task_lock: &Vec<String> = &*tasks.lock().unwrap();
    for contents/*: &String*/ in task_lock {
        println!("{contents}");
    }
}
