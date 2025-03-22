// Cargo Crates
use std::collections::HashMap;

/*
 * Traits
 */
trait Accommodation {
    fn book(&mut self, name: &str, nights: u32);
}

trait Description {
    fn get_description(&self) -> String {
        String::from("A wonderful place to stay.")
    }
}

/*
 * Structs
 */

// Hotel
#[derive(Debug)]
struct Hotel {
    name: String,
    reservations: HashMap<String, u32>,
}

impl Hotel {
    fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            reservations: HashMap::new(),
        }
    }

    fn summarize(&self) -> String {
        format!("{}: {}", self.name, self.get_description())
    }
}

impl Accommodation for Hotel {
    fn book(&mut self, name: &str, nights: u32) {
        self.reservations.insert(name.to_string(), nights);
    }
}

impl Description for Hotel {}

// AirBnB
#[derive(Debug)]
struct AirBnB {
    host: String,
    guests: Vec<(String, u32)>,
}

impl AirBnB {
    fn new(host: &str) -> Self {
        Self {
            host: host.to_string(),
            guests: vec![],
        }
    }
}

impl Accommodation for AirBnB {
    fn book(&mut self, name: &str, nights: u32) {
        self.guests.push((name.to_string(), nights));
    }
}

impl Description for AirBnB {
    fn get_description(&self) -> String {
        format!("Please enjoy {}'s apartment.", self.host)
    }
}

// Entity can be of any type as long as it implements the Accommodation trait.
// Syntactic Sugar
//fn book_for_one_night(entity: &mut impl Accommodation, guest: &str) {
//    entity.book(guest, 1);
//}

// Trait Bound (Generic) syntax
fn book_for_one_night<T: Accommodation>(entity: &mut T, guest: &str) {
    entity.book(guest, 1);
}

// Option 1:
//fn mix_and_match(
//    // The first parameter will be a mutable reference to some type that
//    // implements both, Accommodation and Description traits.
//    first: &mut (impl Accommodation + Description),
//    // The second parameter promises to implement only the Accommodation trait.
//    second: &mut impl Accommodation,
//    guest: &str,
//) {
//    first.book(guest, 1);
//    first.get_description();
//
//    second.book(guest, 1);
//}

// Option 2:
//fn mix_and_match<T: Accommodation + Description, U: Accommodation>(
//    first: &mut T,
//    second: &mut U,
//    guest: &str,
//) {
//    first.book(guest, 1);
//    second.book(guest, 1);
//}

// Option 3 (where clause):
fn mix_and_match<T, U>(first: &mut T, second: &mut U, guest: &str)
where
    T: Accommodation + Description,
    U: Accommodation,
{
    first.book(guest, 1);
    second.book(guest, 1);
}

fn choose_best_place_to_stay() -> impl Accommodation + Description {
    Hotel::new("The Luxe")
    //AirBnB::neW("Peter")
}

fn main() {
    let mut hotel = choose_best_place_to_stay();
    let mut airbnb: AirBnB = AirBnB::new("Peter");
    mix_and_match(&mut hotel, &mut airbnb, "Piers");
    //println!("{hotel:#?} {airbnb:#?}");

    //let mut hotel: Hotel = Hotel::new("The Luxe");
    //book_for_one_night(&mut hotel, "Piers");
    //println!("{hotel:#?}");

    //let mut airbnb: AirBnB = AirBnB::new("Peter");
    //book_for_one_night(&mut airbnb, "Amanda");
    //println!("{airbnb:#?}");

    //let mut hotel: Hotel = Hotel::new("The Luxe");
    //println!("{}", hotel.summarize());
    //hotel.book("Piers", 5);
    //println!("{:#?}", hotel);

    //let mut airbnb: AirBnB = AirBnB::new("Peter");
    //println!("{}", airbnb.get_description());
    //airbnb.book("Piers", 3);
    //println!("{:#?}", airbnb);
}
