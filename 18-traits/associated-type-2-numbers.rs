/*
 * Cargo Crates
 */
use std::ops::Add;

/*
 * Utility Functions
 */
fn add_two_numbers<T: Add<Output = T>>(a: T, b: T) -> T {
    a + b
}

fn main() {
    // Numbers:
    let integer_sum: i32 = add_two_numbers(1, 2);
    let float_sum: f64 = add_two_numbers(1.5, 2.4);
    println!("{integer_sum} and {float_sum}");
}
