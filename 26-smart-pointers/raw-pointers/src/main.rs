fn main() {
    let mut sushi = String::from("Yellowtail");

    //let sushi_ref: &String = &sushi;
    //let sushi_ref_2: &String = &sushi;
    //println!("{sushi_ref} and {sushi_ref_2}");

    //let sushi_raw_pointer_1: *const String = &raw const sushi; // Equivalent to a pointer in C
    //let sushi_raw_pointer_2: *const String = &sushi; // Rust will force this regular reference to a raw pointer.

    let sushi_raw_mutable_pointer_1: *mut String = &raw mut sushi;
    let sushi_raw_mutable_pointer_2: *mut String = &raw mut sushi;

    drop(sushi); // De-allocate the data.

    // Unpredicatable code
    unsafe {
        println!(
            "{} {}",
            *sushi_raw_mutable_pointer_1, *sushi_raw_mutable_pointer_2); // Here, we de-reference data that is not there anymore.
    }
}
