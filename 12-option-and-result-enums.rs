#[derive(Debug)]
struct Food {
    name: String,
}

#[derive(Debug)]
struct Restaurant {
    reservations: u32,
    has_mice_infestation: bool,
}

impl Restaurant {
    fn chef_special(&self) -> Option<Food> {
        if self.has_mice_infestation {
            return None;
        }

        let mut option: Option<Food>;

        if self.reservations < 12 {
            option = Some(
                Food {
                    name: String::from("Uni Sashimi"),
                }
            );
        } 
        else if self.reservations >= 12 {
            option = Some(
                Food {
                    name: String::from("Strip Steak"),
                }
            );
        }
        else {
            option = None;
        }

        option
    }

    fn deliver_burger(&self, address: &str) -> Result<Food, String> {
        if self.has_mice_infestation {
            return Err(
                String::from("Sorry, we have a mice problem.")
            );
        }

        let mut result: Result<Food, String>;

        if address.is_empty() {
            result = Err(
                String::from("No delivery address specified.")
            );
        }
        else {
            result = Ok(
                Food {
                    name: String::from("Burger"),
                }
            );
        }

        result
    }
}

fn main() {
    let restaurant_1: Restaurant = Restaurant {
        reservations: 11,
        has_mice_infestation: true,
    };

    let chef_special_1: Option<Food> = restaurant_1.chef_special();
    println!("{chef_special_1:#?}");

    let deliver_burger_1: Result<Food, String> = restaurant_1
        .deliver_burger("123 Elm Street");
    println!("{deliver_burger_1:#?}");

    let restaurant_2: Restaurant = Restaurant {
        reservations: 15,
        has_mice_infestation: false,
    };

    let chef_special_2: Option<Food> = restaurant_2.chef_special();
    println!("{chef_special_2:#?}");

    let deliver_burger_2_1: Result<Food, String> = restaurant_2
        .deliver_burger("");
    println!("{deliver_burger_2_1:#?}");

    let deliver_burger_2_2: Result<Food, String> = restaurant_2
        .deliver_burger("123 Test Avenue");
    println!("{deliver_burger_2_2:#?}");
}
