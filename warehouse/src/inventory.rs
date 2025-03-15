/*
 * Custom Modules
 */
pub mod products;

/*
 * Public
 */
pub const FLOOR_SPACE: i32 = 10_000;
pub const MANAGER: &str = "Bob's Inventory";

pub fn talk_to_manager() {
    println!("Hey, {}, how's your coffee?", MANAGER);
}
