#[derive(Debug)]
struct Flight {
    origin: String,
    destination: String,
    price: f64,
    passengers: u32,
}

// Associated Functions
impl Flight {
    // Constructor (the name `new` is up tu us)
    fn new(
        origin: String,
        destination: String,
        price: f64,
        passengers: u32
    ) -> Self {
        Self {
            origin,
            destination,
            price,
            passengers,
        }
    }
}

// Methods
impl Flight {
    fn change_destination(&mut self, new_destination: String) -> &mut Flight {
        self.destination = new_destination;

        self
    }

    fn increase_price(&mut self) -> &mut Flight {
        let new_price: f64 = self.price * 1.2;

        self.price = new_price;

        self
    }

    fn itinerary(&self) {
        println!("{} -> {}", self.origin, self.destination);
    }
}

fn main() {
    let mut new_flight: Flight = Flight::new(
      String::from("New York"),
      String::from("Paris"),
      849.99,
      250,
    );

    println!("Before update: {:#?}", new_flight);

    new_flight
        .change_destination(String::from("London"))
        .increase_price()
        .itinerary();

    println!("After update: {:#?}", new_flight);

    let next_flight: Flight = Flight {
      origin: String::from("Amsterdam"),
      destination: String::from("Washington"),
      ..new_flight
    };

    println!("Next flight: {:#?}", next_flight);
}
