/*
 * In Rust, inner functions do not have access to their containing environment.
 * To fix this error, you can replace the function with a closure.
 */
fn main() {
    let multiplier: i32 = 5;

    //fn multiply_by(value: i32) -> i32 {
    //    // error[E0434]: can't capture dynamic environment in a fn item
    //    value * multiplier
    //}

    /*
     * Closure (anonymous function):
     * `let multiply_by: impl Fn() = || {};`
     * Needs to implement the Fn Trait for invokable procedure.
     * `impl Fn(i32) -> i32`
     */
    let multiply_by = |value: i32| -> i32 {
        return value * multiplier;
    };

    println!("{}", multiply_by(2));

    // `impl Fn(i32, i32) -> i32`
    let product = |a: i32, b: i32| -> i32 {
        println!("Calculating product for you:");
        return a * b;
    };

    println!("{}", product(3, 10));
    println!("{}", product(5, 8));
}
