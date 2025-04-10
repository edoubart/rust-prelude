/*
 * External Crates
 */
//use fake::{Fake, Faker};

/*
 * Custom Modules
 */
use warehouse::{
    Item,
    ProductCategory,
    FLOOR_SPACE,
    INVENTORY_MANAGER,
    ORDERS_MANAGER,
};

// Shortcuts
//use std::collections::*; // glob operator (not recommended)
//use std::{
//    fmt,
//    io::{self, stdin, stdout},
//};

fn main() {
    println!(
        "Our managers are {} and {}. We have {} square feet of floor space.",
        INVENTORY_MANAGER, ORDERS_MANAGER, FLOOR_SPACE
    );

    //let fake_item: Item = Faker.fake();
    //println!("{:?}", fake_item);

    //let random_category: ProductCategory = Faker.fake();
    //println!("{:?}", random_category);

    //let favorite_category: ProductCategory = ProductCategory::Hammer;
    //println!("My favorite of item is {favorite_category:?}");

    //let tall_ladder: Item = Item::new(
    //    // Name
    //    String::from("Ladder-o-matic 2000"),
    //    // Category
    //    favorite_category,
    //    // Quantity
    //    100
    //);
    //println!("{:#?}", tall_ladder);
}
