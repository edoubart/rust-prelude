/*
 * Cargo Crates
 */
use std::slice::IterMut;

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
    let mut flavors: [String; 3] = [
        String::from("Chocolate"),
        String::from("Vanilla"),
        String::from("Strawberry"),
    ];

    /*
     * The `.iter_mu()` method will create an iterator that yields mutable
     * references to the collection's elements.
     */
    let iterator: IterMut<'_, String> = flavors.iter_mut();

    for flavor in iterator {
    //for flavor: &mut String in iterator {
    //for flavor in iterator.iter_mut() {
    //for flavor in &mut iterator {
        flavor.push_str(" Ice Cream");
    }

    println!("{:?}", flavors);
    // error[E0382]: borrow of moved value: `iterator`
    //println!("{:?}", iterator);

    let mut school_grades: [i32; 4] = [85, 90, 72, 92];

    for grade in &mut school_grades {
    //for grade: &mut i32 in &mut school_grades {
        // `*` Dereference Operator
        *grade -= 2;
    }

    println!("{:?}", school_grades);
}
