/*
 * Sharing States (Part 1)
 */
use std::sync::Mutex;
use std::sync::MutexGuard;
use std::thread;

fn main() {
    let m: Mutex<i32> = Mutex::new(5);

    {
        let mut num: MutexGuard<i32> = m.lock().unwrap();
        *num = 10;
    }

    let lock_m: MutexGuard<i32> = m.lock().unwrap();
    println!("m is: {:?}", *lock_m);

    drop(lock_m);

    let lock_m1: MutexGuard<i32> = m.lock().unwrap();
    println!("This code is blocked");
}
