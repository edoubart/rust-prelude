/*
 * Custom Modules
 */
mod inventory;
mod orders;

// Shortcuts
use inventory::{
    Item,
    ProductCategory,
    FLOOR_SPACE,
    MANAGER as INVENTORY_MANAGER,
};
use orders::MANAGER as ORDERS_MANAGER;

fn main() {
    println!(
        "Our managers are {} and {}. We have {} square feet of floor space.",
        INVENTORY_MANAGER, ORDERS_MANAGER, FLOOR_SPACE
    );

    let favorite_category: ProductCategory = ProductCategory::Hammer;
    println!("My favorite of item is {favorite_category:?}");

    let tall_ladder: Item = Item::new(
        // Name
        String::from("Ladder-o-matic 2000"),
        // Category
        favorite_category,
        // Quantity
        100
    );
    println!("{:#?}", tall_ladder);
}
