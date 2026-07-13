/*
 * Sharing States (Part 2)
 */
use std::rc::Rc;
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::MutexGuard;
use std::thread;
use thread::JoinHandle;

struct File {
    text: Vec<String>,
}

fn main() {
    let file: Arc<Mutex<File>> = Arc::new(Mutex::new(File { text: vec![] }));
    let mut thread_vec: Vec<JoinHandle<()>> = vec![];

    for i in 0..10 {
        let file: Arc<Mutex<File>> = Arc::clone(/*self:*/ &file);
        let handle: JoinHandle<()> = thread::spawn(move || {
            let mut file_lock: MutexGuard<File> = file.lock().unwrap();
            file_lock.text.push(format!("Hello from Thread {i}"));
        });
        thread_vec.push(handle);
    }

    for handle in thread_vec {
        handle.join().unwrap();
    }

    let file_lock: MutexGuard<File> = file.lock().unwrap();
    for t in &file_lock.text {
        println!("{t}");
    }
}
