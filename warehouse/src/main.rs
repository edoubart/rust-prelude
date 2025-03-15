mod inventory {
    /*
     * Private
     */
    const FLOOR_SPACE: i32 = 10_000;

    /*
     * Public
     */
    pub const MANAGER: &str = "Bob's Inventory";

    #[derive(Debug)]
    enum ProductCategory {
        Ladder,
        Hammer,
    }

    struct Item {
        name: String,
        category: ProductCategory,
        quantity: u32,
    }

    fn talk_to_manager() {
        println!("Hey, {MANAGER}, how's your coffee?");
    }
}

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
