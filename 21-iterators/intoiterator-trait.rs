/*
 * Cargo Crates
 */
use std::collections::HashMap;
use std::collections::hash_map::IntoIter as IntoIterHashMap;
use std::vec::IntoIter as IntoIterVec;

fn main() {
    //let my_vector: Vec<i32> = vec![4, 8, 15, 16, 23, 42];
    //// `pub struct IntoIter<T>`
    //// The IntoIter Struct implements the IntoIter Trait.
    //let my_iterator: IntoIterVec<i32> = my_vector.into_iter();
    //// error[E0382]: borrow of moved value: `my_vector`
    //// `.into_iter()` moves ownership of the vector.
    ////println!("{:#?}", my_vector);

    //let my_vector: Vec<bool> = vec![false, true, false];
    //// `pub struct IntoIter<T>`
    //// The IntoIter Struct implements the IntoIter Trait.
    //let my_iterator: IntoIterVec<bool> = my_vector.into_iter();
    //// error[E0382]: borrow of moved value: `my_vector`
    //// `.into_iter()` moves ownership of the vector.
    ////println!("{:#?}", my_vector);

    let mut my_hashmap: HashMap<&str, i32> = HashMap::new();
    my_hashmap.insert("CBS", 2);
    let my_iterator: IntoIterHashMap<&str, i32> = my_hashmap.into_iter();
}
