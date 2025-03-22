// Cargo Crates
use std::collections::HashMap;
use std::fmt::Display;

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
struct Hotel<T> {
    name: T,
    reservations: HashMap<String, u32>,
}

// For Hotel of type T ...
impl<T> Hotel<T> {
    fn new(name: T) -> Self {
        Self {
            name,
            reservations: HashMap::new(),
        }
    }
}

// For Hotel of type T that implements the Display trait ...
impl<T: Display> Hotel<T> {
    fn summarize(&self) -> String {
        format!("{}: {}", self.name, self.get_description())
    }
}

impl<T> Accommodation for Hotel<T> {
    fn book(&mut self, name: &str, nights: u32) {
        self.reservations.insert(name.to_string(), nights);
    }
}

impl<T> Description for Hotel<T> {}

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
    //AirBnB::new("Peter")
}

fn main() {
    let hotel_1: Hotel<String> = Hotel::new(String::from("The Luxe"));
    println!("{}", hotel_1.summarize());

    let hotel_2: Hotel<&str> = Hotel::new("The Golden Standard");
    println!("{}", hotel_2.summarize());

    let hotel_3: Hotel<Vec<&str>> = Hotel::new(
        vec!["The Sweet Escape", "Hilton Edition"]
    );
    //println!("{}", hotel_3.summarize()); // Doesn't work!
}
