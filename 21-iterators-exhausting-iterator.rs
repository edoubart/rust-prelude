/*
 * Cargo Crates
 */
use std::vec::IntoIter as IntoIterVec;

fn main() {
    let my_vector: Vec<i32> = vec![4, 8, 15, 16, 23, 42];
    // `pub struct IntoIter<T>`
    // The IntoIter Struct implements the IntoIter Trait.
    let mut my_iterator: IntoIterVec<i32> = my_vector.into_iter();
    // error[E0382]: borrow of moved value: `my_vector`
    // `.into_iter()` moves ownership of the vector.
    //println!("{:?}", my_vector);
    println!("{:?}", my_iterator); // -> IntoIter([4, 8, 15, 16, 23, 42])

    println!("{:?}", my_iterator.next()); // -> Some(4)
    println!("{:?}", my_iterator);        // -> IntoIter([8, 15, 16, 23, 42])

    println!("{:?}", my_iterator.next()); // -> Some(8)
    println!("{:?}", my_iterator);        // -> IntoIter([15, 16, 23, 42])

    println!("{:?}", my_iterator.next()); // -> Some(15)
    println!("{:?}", my_iterator);        // -> IntoIter([16, 23, 42])

    println!("{:?}", my_iterator.next()); // -> Some(16)
    println!("{:?}", my_iterator);        // -> IntoIter([23, 42])

    println!("{:?}", my_iterator.next()); // -> Some(23)
    println!("{:?}", my_iterator);        // -> IntoIter([42])

    println!("{:?}", my_iterator.next()); // -> Some(42)
    println!("{:?}", my_iterator);        // -> IntoIter([])

    println!("{:?}", my_iterator.next()); // -> None
    println!("{:?}", my_iterator);        // -> IntoIter([])

    println!("{:?}", my_iterator.next()); // -> None
    println!("{:?}", my_iterator);        // -> IntoIter([])
}
