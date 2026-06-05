use rand::random;
use rand::{rng, Rng};
use rand::rngs::ThreadRng;
use rand::seq::SliceRandom;

fn main() {
    let random_float: f64 = random::<f64>();
    println!("{}", random_float * 100.0);
    
    let random_int: u8 = random::<u8>();
    println!("{}", random_int);

    let mut my_rng: ThreadRng = rng();
    let random_float: f64 = my_rng.random::<f64>();
    println!("{}", random_float);

    let ten_random_values: Vec<i8> = (0..10)
        .map(|_| my_rng.random::<i8>())
        .collect::<Vec<i8>>();
    println!("{:?}", ten_random_values);

    let random_number: i32 = my_rng.random_range(29..53);
    println!("{random_number}");

    println!("{}", my_rng.random_bool(0.9)); // 90% chance of getting a true

    println!("{}", my_rng.random_ratio(1, 2)); // A 1 in 2 chance of getting a true

    let mut candies: Vec<&str> = vec![
        "Sour Patch Kids",
        "Kit Kat",
        "Twix",
        "Snickers",
        "Starbust"
    ];
    candies.shuffle(&mut my_rng); // Here, we pass a mutable reference the Random Number Generator.
    println!("{:?}", candies);
}
