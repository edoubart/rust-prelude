#![allow(unused_variables)]

fn main() {
    let price: i32 = 1_337;
    let price: i16 = price as i16;

    let new_price: f64 = 1_299.99;
    println!("The new price is {new_price:.3}.");

    let with_milk: bool = false;
    let with_sugar: bool = false;
    let is_my_type_of_coffee: bool = with_milk && with_sugar;
    let is_acceptable_coffee: bool = with_milk || with_sugar;

    let my_array: [i8; 4] = [15, 75, 23, 14];
    dbg!(my_array);

    let product: (i32, f64, bool, &[i8; 4]) = (1, 9.99, false, &my_array);
    dbg!(product);
}
