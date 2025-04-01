/*
 * Cargo Crates
 */
use std::ops::Add;

/*
 * Structs
 */
#[derive(Debug)]
struct Lunch {
    cost: f64,
}

/*
 * Implement the Add Trait for Struct:
 *
 * fn add(self, rhs: Rhs) -> Self::Output;
 */
impl Add for Lunch {
    // Associated Type
    type Output = Lunch;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            cost: self.cost + rhs.cost,
        }
    }

    // Associated Type
    //type Output = f64;

    //fn add(self, rhs: Self) -> Self::Output {
    //    self.cost + rhs.cost
    //}
}

fn main() {
    // Lunches
    let lunch_1: Lunch = Lunch { cost: 19.99 };
    let lunch_2: Lunch = Lunch { cost: 29.99 };
    println!("{:?}", lunch_1 + lunch_2);
}
