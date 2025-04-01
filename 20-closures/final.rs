#[derive(Debug)]
struct SupermarketItem {
    name: String,
    price: f64,
}

#[derive(Debug)]
struct ShoppingCart<'a> {
    items: &'a mut Vec<SupermarketItem>,
}

impl<'a> ShoppingCart<'a> {
    fn traverse_items<F>(&mut self, mut operation: F)
    where
        F: FnMut(&mut SupermarketItem)
    {
        let final_index: usize = self.items.len() - 1;
        let mut current_index: usize = 0;
        while current_index <= final_index {
            let mut current_item: &mut SupermarketItem =
                &mut self.items[current_index];
            operation(&mut current_item);
            current_index += 1;
        }
    }

    fn checkout<F>(self, operation: F)
    where
        F: FnOnce(ShoppingCart)
    {
        operation(self);
    }
}

fn main() {
    let mut supermarket_items: Vec<SupermarketItem> = vec![
        SupermarketItem {
            name: String::from("APPLE"),
            price: 3.99,
        },
        SupermarketItem {
            name: String::from("BANANA"),
            price: 2.99,
        }
    ];
    let mut shopping_cart: ShoppingCart = ShoppingCart {
        items: &mut supermarket_items,
    };

    println!("Shopping Cart (before): {:#?}", shopping_cart);

    let closure_1 = |supermarket_item: &mut SupermarketItem| {
        // Decrease the item's price by 15%.
        supermarket_item.price *= 0.85;
    };
    shopping_cart.traverse_items(closure_1);

    println!("Shopping Cart (after price decrease): {:#?}", shopping_cart);

    let closure_2 = |supermarket_item: &mut SupermarketItem| {
        // Lowercase the item's name.
        supermarket_item.name = supermarket_item.name.to_lowercase();
    };
    shopping_cart.traverse_items(closure_2);

    println!("Shopping Cart (after name lowercase): {:#?}", shopping_cart);

    let mut total_price: f64 = 0.0;
    let closure_3 = |mut shopping_cart: ShoppingCart| {
        println!("Shopping Cart: {:#?}", shopping_cart);

        let closure_4 = |supermarket_item: &mut SupermarketItem| {
            total_price += supermarket_item.price;
        };
        shopping_cart.traverse_items(closure_4);
    };
    shopping_cart.checkout(closure_3);

    println!("Total Price: ${:.2}", total_price);
}
