/*
 * Cargo Crates
 */
use std::fmt::{
    Debug,
    Display,
    Formatter,
    Result
};
use std::fs;
use std::ops::Drop;

/*
 * Enums
 */
enum AppleType {
    RedDelicious,
    GrannySmith,
}

/*
 * Implement the Display Trait on an Enum:
 *
 * pub trait Display {
 *     // Required method
 *     fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error>;
 * }
 */
impl Display for AppleType {
    // Custom implementation of the `fmt` ("format") function to add support for
    // the Display trait.
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
        match self {
            AppleType::RedDelicious => {
                write!(
                    formatter,
                    "`Red Apple emoji` Red Delicious `Red Apple emoji`"
                )
            }
            AppleType::GrannySmith => {
                write!(
                    formatter,
                    "`Green Apple emoji` Granny Smith `Green Apple emoji`"
                )
            }
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
impl Debug for AppleType {
    // Custom implementation of the `fmt` ("format") function to add support for
    // the Debug trait.
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
        match self {
            AppleType::RedDelicious => {
                write!(formatter, "AppleType::RedDelicious")
            }
            AppleType::GrannySmith => {
                write!(formatter, "AppleType::GrannySmith")
            }
        }
    }
}

/*
 * Structs
 */
struct Apple {
    kind: AppleType,
    price: f64,
}

/*
 * Implement the Display Trait on Struct:
 *
 * pub trait Display {
 *     // Required method
 *     fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error>;
 * }
 */
impl Display for Apple {
    // Custom implementation of the `fmt` ("format") function to add support for
    // the Display trait.
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
        write!(formatter, "One {} for ${:.2}.", self.kind, self.price)
    }
}

/*
 * Implement the Debug Trait on Struct:
 *
 * pub trait Debug {
 *     // Required method
 *     fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error>;
 * }
 */
//impl Debug for Apple {
//    // Custom implementation of the `fmt` ("format") function to add support for
//    // the Debug trait.
//    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
//        write!(
//            formatter,
//            "Apple ::: [ Kind: {:#?}, Price: {} ]",
//            self.kind, self.price
//        )
//    }
//}

impl Debug for Apple {
    // Custom implementation of the `fmt` ("format") function to add support for
    // the Debug trait.
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
        // Implements the Builder design pattern
        formatter
            .debug_struct("** Apple **") // Returns a DebugStruct
            .field("Kind", &self.kind) // Returns a DebugStruct
            .field("Price", &self.price) // Returns a DebugStruct
            .finish() // Returns a Result
    }
}

/*
 * Implement the Drop Trait on Struct (whenever you have some cleanup operation
 * to do):
 *
 * pub trait Drop {
 *     // Required method
 *     fn drop(&mut self);
 * }
 */
impl Drop for Apple {
    fn drop(&mut self) {
        match fs::remove_file("apple.txt") {
            Ok(_) => println!("Goodbye, my sweet apple."),
            Err(error) => eprintln!("Error deleting file: {error}"),
        }
    }
}

fn main() {
    let lunch_snack: Apple = Apple {
        kind: AppleType::GrannySmith,
        price: 1.04,
    };
    // Display
    println!("{}", lunch_snack);
    // Debug
    println!("{:#?}", lunch_snack);

    let dinner_snack: Apple = Apple {
        kind: AppleType::RedDelicious,
        price: 1.15,
    };
    // Display
    println!("{}", dinner_snack);
    // Debug
    println!("{:#?}", dinner_snack);
}
