 use std::ops::Deref;

/*
 * When given a reference to a type that implements the *Deref* trait, Rust will
 * convert it into a reference of another type if necessary.
 */

fn main() {
    let text: String = String::from("Hello");
    output_text(&text); // &String -> &str

    let text: String = String::from("Hello");
    let the_slice = text.deref();
    output_text(the_slice); // &String -> &str

    let text: String = String::from("Hello");
    let my_box: Box<String> = Box::new(text);
    output_text(&my_box); // &Box -> Box.deref() -> &String -> String.deref() -> &str

    let value: &str = &(*my_box)[..];
    output_text(value);
}

fn output_text(text: &str) {
    println!("{}", text);
}
