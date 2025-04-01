/*
 * Structs
 */
#[derive(Debug, Clone)]
struct Duration {
    hours: u32,
    minutes: u32,
    seconds: u32,
}

impl Duration {
    fn new(hours: u32, minutes: u32, seconds: u32) -> Self {
        Self {
            hours,
            minutes,
            seconds,
        }
    }
}

/*
 * Implement the Copy Supertrait for Struct:
 *
 * pub trait Copy: Clone {}
 *
 * Copy Trait (Supertrait) > Clone Trait (Subtrait)
 * Clone Trait -> `.clone()` method
 * Copy Trait -> `=` operator
 * To implement the Copy Trait, you need to make sure that you have support for
 * the implementation of the Clone Trait recursively.
 */
impl Copy for Duration {}

fn main() {
    let one_hour: Duration = Duration::new(1, 0, 0);
    let another_hour: Duration = one_hour;
    println!("{:#?}", one_hour);
}
