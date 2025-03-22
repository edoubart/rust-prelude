// Cargo Crates
use std::collections::HashMap;

/*
 * Traits
 */
trait Accommodation {
    fn get_description(&self) -> String {
        String::from("A wonderful place to stay.")
    }
    fn book(&mut self, name: &str, nights: u32);
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
    //fn get_description(&self) -> String {
    //    format!("{} is the pinnacle of luxury.", self.name)
    //}

    fn book(&mut self, name: &str, nights: u32) {
        self.reservations.insert(name.to_string(), nights);
    }
}

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
    fn get_description(&self) -> String {
        format!("Please enjoy {}'s apartment.", self.host)
    }

    fn book(&mut self, name: &str, nights: u32) {
        self.guests.push((name.to_string(), nights));
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
fn mix_and_match(
    first: &mut impl Accommodation,
    second: &mut impl Accommodation,
    guest: &str
) {
    first.book(guest, 1);
    second.book(guest, 1);
}

// Option 2:
//fn mix_and_match<T: Accommodation, U: Accommodation>(
//    first: &mut T,
//    second: &mut U,
//    guest: &str
//) {
//    first.book(guest, 1);
//    second.book(guest, 1);
//}

fn main() {
    let mut hotel: Hotel = Hotel::new("The Luxe");
    let mut airbnb: AirBnB = AirBnB::new("Peter");
    mix_and_match(&mut hotel, &mut airbnb, "Piers");
    println!("{hotel:#?} {airbnb:#?}");

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
