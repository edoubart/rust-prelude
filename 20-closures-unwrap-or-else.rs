/*
 * pub fn unwrap_or_else<F>(self, f: F) -> T
 * where
 *     F: FnOnce() -> T,
 * {
 *     match self {
 *         Some(x: T) => x,
 *         None => f(),
 *     }
 * }
 */

fn main() {
    let option: Option<&str> = Some("Salami");
    let closure = || "Pizza";
    let food: &str = option.unwrap_or_else(closure);
    println!("{food}");

    let option: Option<&str> = None;
    let pizza_fan: bool = false;
    let closure = || {
        if pizza_fan {
            "Pizza"
        } else {
            "Hot Pockets"
        }
    };
    let food: &str = option.unwrap_or_else(closure);
    println!("{food}");
}
