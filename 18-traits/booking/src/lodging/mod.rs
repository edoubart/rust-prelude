// Cargo Crates
use std::collections::HashMap;
use std::fmt::Display;

/*
 * Traits
 */
pub trait Accommodation {
    fn book(&mut self, name: &str, nights: u32);
}

pub trait Description {
    fn get_description(&self) -> String {
        String::from("A wonderful place to stay.")
    }
}

/*
 * Structs
 */
#[derive(Debug)]
pub struct Hotel<T> {
    name: T,
    reservations: HashMap<String, u32>,
}

// For Hotel of type T ...
impl<T> Hotel<T> {
    pub fn new(name: T) -> Self {
        Self {
            name,
            reservations: HashMap::new(),
        }
    }
}

// For Hotel of type T that implements the Display trait ...
impl<T: Display> Hotel<T> {
    pub fn summarize(&self) -> String {
        format!("{}: {}", self.name, self.get_description())
    }
}

impl<T> Accommodation for Hotel<T> {
    fn book(&mut self, name: &str, nights: u32) {
        self.reservations.insert(name.to_string(), nights);
    }
}

impl<T> Description for Hotel<T> {}

#[derive(Debug)]
pub struct AirBnB {
    host: String,
    guests: Vec<(String, u32)>,
}

impl AirBnB {
    pub fn new(host: &str) -> Self {
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
