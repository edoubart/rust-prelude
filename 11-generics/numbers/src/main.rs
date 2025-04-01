/***********
 * Imports *
 ***********/
use num_traits::{ToPrimitive};

/*
 * Generic Types:
 * Makes code more flexible when working with types.
 * Like an argument list, but for types.
 * 'T' (= type "variable")
 * 'Float' is a *trait*.
 * Here, it is being used as a *trait bound*.
 *
 * Traits:
 *   - A trait is a set of methods;
 *   - It can contain *abstract methods* which don't have an implementation;
 *   - It can contain *default methods* which have an implementation;
 *   - A struct/enum/primitive can *implement* a trait;
 *   - The implementor has to provide an implementation for all of the
 * *abstract methods*;
 *   - The implementor can *optionally* override the default methods.
 */
// First version: we can pass in BOTH, f32 of f64 numbers.
// Second version: we can pass in any type of numbers.
fn solve<T: ToPrimitive, U: ToPrimitive>(a: T, b: U) -> f64 {
    let a_f64 = a.to_f64().unwrap();
    let b_f64 = b.to_f64().unwrap();

    (a_f64.powi(2) + b_f64.powi(2)).sqrt()
}

/*****************
 * Main Function *
 *****************/
fn main() {
    // f32
    let a: u8 = 3;
    //let b: f32 = 4.0;

    // f64
    //let a: f64 = 3.0;
    let b: f64 = 4.0;

    // f32
    //println!("{}", solve::<f32>(a, b))

    // f64
    //println!("{}", solve::<f64>(a, b))

    // Common (inference)
    println!("{}", solve(a, b))
}
