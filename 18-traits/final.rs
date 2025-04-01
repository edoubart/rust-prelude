/*
 * Cargo Crates
 */
use std::fmt::{Debug, Display, Formatter};

/*
 * Traits
 */
trait Drinkable {
    fn consume(&mut self);

    // Getters
    fn get_data(&self) -> String;

    fn stats(&self) {
        println!("{}", self.get_data());
    }
}

/*
 * Enums
 */
#[derive(Debug)]
enum Milk {
    Whole,
    Oat,
    Almond,
}

/*
 * Structs
 */
struct Coffee<T> {
    kind: T,
    milk: Milk,
    ounces: u32,
}

impl<T> Coffee<T> {
    // Constructor
    fn new(kind: T, milk: Milk, ounces: u32) -> Self {
        Self {
            kind,
            milk,
            ounces,
        }
    }
}

/*
 * Implement the Debug Trait on an Enum:
 *
 * pub trait Debug {
 *     // Required method
 *     fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error>;
 * }
 */
impl<T: Debug> Debug for Coffee<T> {
    // Custom implementation of the `fmt` ("format") function to add support for
    // the Debug trait.
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        // Implements the Builder design pattern
        formatter
            .debug_struct("COFFEE")        // Returns a DebugStruct
            .field("Kind", &self.kind)     // Returns a &mut DebugStruct
            .field("Milk", &self.milk)     // Returns a &mut DebugStruct
            .field("Ounces", &self.ounces) // Returns a &mut DebugStruct
            .finish()                      // Returns a Result
    }
}

impl<T: Display> Drinkable for Coffee<T> {
    fn consume(&mut self) {
        self.ounces = 0;
    }

    // Getters
    fn get_data(&self) -> String {
        format!("A delicious {} ounce {}.", self.ounces, self.kind)
    }
}

#[derive(Debug)]
struct Soda {
    calories: u32,
    price: f64,
    flavor: String,
    percentage: u32,
}

impl Soda {
    // Constructor
    fn new(calories: u32, price: f64, flavor: String) -> Self {
        Self {
            calories,
            price,
            flavor,
            percentage: 100,
        }
    }
}

/*
 * Implement the Clone Trait on Struct:
 *
 * pub trait Clone: Sized {
 *     // Required method
 *     fn clone(&self) -> Self;
 *
 *     // Provided method
 *     fn clone_from(&mut self, source: &Self) { ... }
 * }
 */
impl Clone for Soda {
    fn clone(&self) -> Self {
        Self {
            calories: self.calories,
            price: self.price,
            flavor: self.flavor.clone(),
            percentage: self.percentage,
        }
    }
}

/*
 * Implement the Display Trait on an Enum:
 *
 * pub trait Display {
 *     // Required method
 *     fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error>;
 * }
 */
impl Display for Soda {
    // Custom implementation of the `fmt` ("format") function to add support for
    // the Display trait.
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "** {} Soda **", self.flavor)
    }
}

impl Drinkable for Soda {
    fn consume(&mut self) {
        self.percentage = 0;
    }

    // Getters
    fn get_data(&self) -> String {
        format!("Flavor: {}, Calories: {}", self.flavor, self.calories)
    }
}

/*
 * Implement the PartialEq Trait for Struct:
 *
 * pub trait PartialEq<Rhs = Self>
 * where
 *     Rhs: ?Sized,
 * {
 *     // Required method
 *     fn eq(&self, other: &Rhs) -> bool;
 *
 *     // Provided method
 *     fn ne(&self, other: &Rhs) -> bool { ... }
 * }
 */
impl PartialEq for Soda {
    fn eq(&self, other: &Self) -> bool {
        // Two sodas are considered equal if they have the same price.
        self.price == other.price
    }
}

impl Eq for Soda {}

fn main() {
    // Coffees:
    let mut latte: Coffee<&str> = Coffee::new(
        // kind: T
        "Latte",
        // milk: Milk
        Milk::Almond,
        // ounces: u32
        10,
    );
    println!("Latte (before): {:#?}", latte);
    latte.consume();
    println!("Latte (after): {:#?}", latte);

    let mut cappuccino: Coffee<String> = Coffee::new(
        // kind: T
        String::from("Cappuccino"),
        // milk: Milk
        Milk::Whole,
        // ounces: u32
        12,
    );
    println!("Cappuccino: {}", cappuccino.get_data());

    // Sodas:
    let pepsi: Soda = Soda::new(
        // calories: u32
        1000,
        // price: f64
        1.99,
        // flavor: String
        String::from("Cherry"),
    );
    println!("Pepsi: {}", pepsi);

    let mut coke: Soda = pepsi.clone();

    println!("{}", pepsi == coke);
    println!("{}", pepsi.eq(&coke));

    println!("Coke (before): {:#?}", coke);
    coke.consume();
    println!("Coke (after): {:#?}", coke);
}
