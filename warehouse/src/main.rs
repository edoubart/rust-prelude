/*
 * Custom Modules
 */
mod inventory;
mod orders;

fn main() {
    println!(
        "Our managers are {} and {}. We have {} square feet of floor space.",
        inventory::MANAGER,
        orders::MANAGER,
        inventory::FLOOR_SPACE,
    );

    inventory::talk_to_manager();

    let favorite_category = inventory::products::ProductCategory::Hammer;
    println!("My favorite of item is {favorite_category:?}");

    let tall_ladder = inventory::products::Item {
        name: String::from("Ladder-o-matic 2000"),
        category: favorite_category,
        quantity: 100,
    };
    println!("{:#?}", tall_ladder);
}
