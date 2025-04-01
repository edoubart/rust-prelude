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
     * Traits: Fn < FnMut < FnOnce
     */
    let multiply_by = |value: i32| -> i32 {
        return value * multiplier;
    };

    /*
     * Shortcuts:
     */
    //let multiply_by = |value: i32| value * multiplier;
    //let multiply_by = |value| value * multiplier as i32;
    // Type inferred from first invokation (!= Generics)
    let multiply_by = |value| value * multiplier; 
    // let mirror = |value| value;

    println!("{}", multiply_by(2));

    // `impl Fn(i32, i32) -> i32`
    let product = |a: i32, b: i32| -> i32 {
        println!("Calculating product for you:");
        return a * b;
    };

    println!("{}", product(3, 10));
    println!("{}", product(5, 8));

    // Vectors don't implement the Copy Trait!
    let numbers_1: Vec<i32> = vec![4, 8, 15, 16, 23, 42];
    println!("{:?}", numbers_1);

    // `impl Fn()`: : Our closure captures an immutable reference to a value.
    let print_numbers = || println!("{:?}", numbers_1);
    print_numbers();
    print_numbers();
    print_numbers();
    println!("{:?}", numbers_1);

    let mut numbers_2: Vec<i32> = vec![4, 8, 15, 16, 23, 42];
    // `impl FnMut()`: Our closure captures an mutable reference to a value.
    // The closure changes everytime it's invoked that's why we mark it with the
    // `mut` keyword.
    let mut add_number = || numbers_2.push(100);
    add_number();
    // The closure "redeclares" itself on each invocation.
    add_number();
    println!("{:?}", numbers_2);

    let number_3: i32 = 13;
    // `impl Fn() -> i32`: Numbers implement the Copy Trait.
    let capture_number = || number_3;
    let a: i32 = capture_number();
    let b: i32 = capture_number();
    println!("{a} {b} {number_3}");

    let first_name_1: String = String::from("Bob");
    // `impl FnOnce() -> String`: Heap allocated Strings do not implement the
    // Copy Trait.
    let capture_string_1 = || first_name_1;
    //println!("{first_name_1}");
    //let owner: String = capture_string_1();

    let first_name_2: String = String::from("Alice");
    // `impl FnOnce() -> String`: Heap allocated Strings do not implement the
    let capture_string_2 = || {
        let person: String = first_name_2;
        println!("{person}");
    };
    capture_string_2();
    // error[E0382]: use of moved value: `capture_string_2` 
    // note: closure cannot be invoked more than once because it moves the
    // variable `first_name_2` out of its environment
    //capture_string_2();

    let first_name_3: String = String::from("John");
    let last_name_3: String = String::from("Doe");
    // `impl Fn()`
    // `move` forces the movement of ownership (of everything).
    let capture_string_3 = move || {
        println!("{first_name_3}");
        println!("{last_name_3}");
    };
    capture_string_3();
    // How?
    capture_string_3();
    // -> https://stackoverflow.com/a/50143627
    capture_string_3();
    //println!("{first_name_3} {last_name_3}");
}
