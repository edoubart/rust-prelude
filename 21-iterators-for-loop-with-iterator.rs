/*
 * Cargo Crates
 */
use std::vec::IntoIter as IntoIterVec;

/*
 * 1. Move of Ownership:
 *   - `for element in collection`
 *   - `for element in collection.into_iter()
 *
 * 2. Borrowing of Immutable References:
 *   - `for element in &collection`
 *   - `for element in collection.iter()`
 *
 * 3. Borrowing of Mutable References:
 *   - `for element in &mut collection`
 *   - `for element in collection.iter_mut()`
 */
fn main() {
    let my_vector: Vec<i32> = vec![4, 8, 15, 16, 23, 42];
    // `pub struct IntoIter<T>`
    // The IntoIter Struct implements the IntoIter Trait.
    let my_iterator: IntoIterVec<i32> = my_vector.into_iter();
    // error[E0382]: borrow of moved value: `my_vector`
    // `.into_iter()` moves ownership of the vector.
    //println!("{:?}", my_vector);
    println!("{:?}", my_iterator); // -> IntoIter([4, 8, 15, 16, 23, 42])

    // Ownership moves from the iteration to the for loop.
    // Rebinds implicitly the iterator from immutable to mutable.
    for number in my_iterator {
        println!("{}", number);
    }

    // error[E0382]: borrow of moved value: `my_iterator`
    //println!("{:?}", my_iterator);

    // If not provided an iterator, the for loop will implicitly call the
    // `.into_iter()` method on whatever is provided.
    // `for number in my_vector { ... `
}
