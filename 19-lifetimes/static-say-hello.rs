const COUNT: i32 = 400;

// Exception lifetime (static):
fn say_hello() -> &'static str {
    // Guaranteed to exist the entire lifetime of the program.
    "Hello" // String Literal
}

// Exception lifetime (static):
fn value() -> &'static i32 {
    // Guaranteed to exist the entire lifetime of the program.
    &COUNT // Constant
}

fn main() {
    let greeting: &str = say_hello();
    println!("{greeting}");

    let count: &i32 = value();
    println!("{count}");
}
