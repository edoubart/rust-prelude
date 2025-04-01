// Custom Modules
use booking::lodging::{
    // Traits
    Accommodation,
    Description,

    // Structs
    AirBnB,
    Hotel,
};
use booking::utils;

fn main() {
    let mut hotel: Hotel<String> = Hotel::new(String::from("The Luxe"));
    let mut airbnb: AirBnB = AirBnB::new("Peter");

    // Dynamic Dispatch (at runtime). Only work with references.
    // We refer to `&hotel` and `&airbnb` as dynamic Trait Objects.
    let stays: Vec<&dyn Description> = vec![&hotel, &airbnb];
    println!("{}", stays[0].get_description());
    println!("{}", stays[1].get_description());

    let mut stays: Vec<&mut dyn Accommodation> = vec![&mut hotel, &mut airbnb];
    stays[0].book("Piers", 2);
    stays[1].book("Amanda", 3);
    println!("{:#?}", hotel);
    println!("{:#?}", airbnb);
}
