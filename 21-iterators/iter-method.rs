/*
 * Cargo Crates
 */
use std::slice::Iter;

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
    /*
     * The `.iter()` method will create an iterator that yields immutable
     * references to the collection's elements.
     */
    let my_iterator: Iter<'_, i32> = my_vector.iter();

    for number in my_iterator {
    //for number: &i32 in my_iterator {
        println!("{}", number);
    }

    println!("{:?}", my_vector);
    // error[E0382]: borrow of moved value: `my_iterator`
    //println!("{:?}", my_iterator);

    // Equivalent:
    //`for number in &my_vector { ... }`

    let cities: Vec<String> = vec![
        String::from("Phoenix"),
        String::from("Dallas"),
    ];

    for city in &cities {
    //for city: &String in &cities {
    //for city: &String in cities.iter() {
        println!("{}", city);
    }
}
