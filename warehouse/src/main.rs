/*
 * Custom Modules
 */
mod inventory;
mod orders;

fn main() {
    println!("The manager of our inventory is {}.", inventory::MANAGER);
    println!("The manager of our orders is {}.", orders::MANAGER);
}
