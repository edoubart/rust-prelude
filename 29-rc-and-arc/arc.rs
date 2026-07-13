/*
 * Atomic Reference Counting (Arc) Smart Pointer
 *
 * ----------------------------------------------------------------------------------------------------
 * Concept / Topic       | Explanation
 * ----------------------------------------------------------------------------------------------------
 * Rc Smart Pointer      | It enables multiple ownership in single-threaded contexts.
 *                       | Rc::new creates a new reference-counted value on the heap.
 *                       | Rc::clone creates a new owner and it does not copy the inner data.
 *                  
 * Strong Count Behavior | Rc::strong_count returns the number of active owners.
 *                       | Each clone increases the strong count.
 *                       | When an owner goes out of scope, the count decreases automatically.
 *                       | The heap data is freed only hen the count reaches zero.
 * ----------------------------------------------------------------------------------------------------
 * Arc Smart Pointer     | It enables multiple ownership in multi-threaded contexts.
 *                       | Arc::new creates a new reference-counted value on the heap.
 *                       | Arc::clone creates a new owner, incrementing the reference count atomically.
 *                       | It is thread-safe and suitable for sharing data across threads.
 * ----------------------------------------------------------------------------------------------------
 */
use std::sync::Arc;
use std::thread;

fn main() {
    let data = Arc::new(vec![1, 2, 3, 4, 5]);

    let mut handles = vec![];

    for i in 0..5 {
        let data_cloned = Arc::clone(&data);
        let handle = thread::spawn(move || {
            println!("Thread {}: {:?}", i, data_cloned);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
}
