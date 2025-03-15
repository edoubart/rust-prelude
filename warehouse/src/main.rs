mod inventory;

mod orders {
    /*
     * Public
     */
    pub const MANAGER: &str = "Alice Doe";
}

fn main() {
    println!("The manager of our inventory is {}.", inventory::MANAGER);
    println!("The manager of our orders is {}.", orders::MANAGER);
}
