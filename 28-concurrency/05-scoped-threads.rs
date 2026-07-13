/*
 * Scoped Threads
 */
use std::thread::{self, JoinHandle, Scope};
//use thread::Scope;

fn main() {
    let mut vec: Vec<i32> = vec![1, 2, 3];

    //thread::spawn(move || {
    //    println!("{:?}", vec);
    //});

    thread::scope(|some_scope: &Scope| {
        some_scope.spawn(|| {
            println!("Thread inside scope");
            println!("vec: {:?}", vec);
        });

        some_scope.spawn(|| {
            println!("Another thread inside scope");
            //vec.push(4); // error[E0502]: cannot borrow `vec` as mutable because it is also borrowed as immutable
            println!("vec: {:?}", vec);
        });
    });

    //let handle_1: JoinHandle<()> = thread::spawn(|| {
    //    println!("Thread inside scope");
    //    println!("vec: {:?}", vec);
    //});

    //let handle_2: JoinHandle<()> = thread::spawn(|| {
    //    println!("Another thread inside scope");
    //    //vec.push(4);
    //    println!("vec: {:?}", vec);
    //});

    //handle_1.join();
    //handle_2.join();

    println!("The scope finished");
    vec.push(5);
    println!("vec: {:?}", vec);
}
