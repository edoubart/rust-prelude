/*
 * Custom Modules
 */
mod inventory;
mod orders;

// Shortcuts
use inventory::products::{Item, ProductCategory};
use inventory::{talk_to_manager, FLOOR_SPACE};

fn main() {
    println!(
        "Our managers are {} and {}. We have {} square feet of floor space.",
        inventory::MANAGER,
        orders::MANAGER,
        FLOOR_SPACE,
    );

    talk_to_manager();

    let favorite_category: ProductCategory = ProductCategory::Hammer;
    println!("My favorite of item is {favorite_category:?}");

    let tall_ladder: Item = Item {
        name: String::from("Ladder-o-matic 2000"),
        category: favorite_category,
        quantity: 100,
    };
    println!("{:#?}", tall_ladder);
}
