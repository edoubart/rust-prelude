/*
 * Cargo Crates
 */
use std::collections::HashSet;

fn main() {
    let numbers: Vec<i32> = vec![4, 8, 15, 16, 23, 42];

    /*
     * The `.collect()` method exhausts the iterator and gathers the resulting
     * values in a new collection type.
     */
    let squares = numbers
    //let squares: Vec<_> = numbers
    //let squares: Vec<i32> = numbers
        .iter()
        .map(|number: &i32| number.pow(2))
        .collect::<HashSet<i32>>();
        //.collect::<Vec<i32>>();
        //.collect();

    println!("{:?}", squares);
    println!("{:?}", numbers);
}
