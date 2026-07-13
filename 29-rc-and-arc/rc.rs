/*
 * Reference Counting (Rc) Smart Pointer
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
use std::rc::Rc;

//enum List {
//    Cons(i32, Option<Box<List>>),
//}
//
//fn main() {
//    let a: List = List::Cons(1, Some(Box::new(List::Cons(2, None))));
//    let b: List = List::Cons(3, Some(Box::new(a))); // - value moved here
//    let c: List = List::Cons(4, Some(Box::new(a))); // error[E0382]: use of moved value: `a`
//}

enum List {
    Cons(i32, Option<Rc<List>>),
}

fn main() {
    let a: Rc<List> = Rc::new(List::Cons(1, Some(Rc::new(List::Cons(2, None)))));
    println!("Reference count after a: {}", Rc::strong_count(&a));

    {
        let b: List = List::Cons(3, Some(Rc::clone(&a)));
        println!("Reference count after b: {}", Rc::strong_count(&a));

        let c: List = List::Cons(4, Some(Rc::clone(&a)));
        println!("Reference count after c: {}", Rc::strong_count(&a));
    }
    println!("Reference count after c: {}", Rc::strong_count(&a));
}
