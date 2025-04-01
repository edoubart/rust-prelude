/***********
 * Imports *
 ***********/
mod basket;
mod container;
mod stack;

use basket::Basket;
use container::Container;
use stack::Stack;

/*
 * Methods for Basket and Stack work differently, but have the same signature;
 * We can define these methods in a trait, then have each struct implement that
 * trait;
 * Benefit: throughout our app, we can work with a Basket or Stack by using
 * trait bounds.
 */

fn add_string<T: Container<String>>(container: &mut T, string: String) {
    container.put(string);
}

/*****************
 * Main Function *
 *****************/
fn main() {
    let mut b1 = Basket::new(String::from("hi there"));
    let b2 = Basket::new(10);
    let b3 = Basket::new(true);

    let mut s1 = Stack::new(vec![String::from("hi")]);
    let s2 = Stack::new(vec![1, 2, 3]);

    add_string(&mut b1, String::from("hi"));
    add_string(&mut s1, String::from("hi"));
}
