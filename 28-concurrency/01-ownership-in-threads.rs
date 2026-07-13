/*
 * Ownership and Threads
 *  - Prerequisites: Closures
 */
use std::thread;

fn main() {
    let x: String = "some string".to_string();
    //let x: i32 = 5;

    // error[E0373]: closure may outlive the current function, but it borrows `x`, which is owned by the current function
    //thread::spawn(|| {
    //    println!("{x}");
    //});

    //thread::spawn(move || {
    //    println!("{x}");
    //});

    // FnOnce trait
    thread::spawn(|| {
        let y: String = x;
        println!("{y}");
    });
   
    // error[E0382]: borrow of moved value: `x`
    //println!("{x}");
}
