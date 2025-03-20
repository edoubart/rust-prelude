// Cargo Crates
use std::collections::HashMap;

fn main() {
    let mut sauces_to_meals: HashMap<&str, Vec<&str>> = HashMap::from([
        ("Ketchup", vec!["French Fries", "Burgers", "Hot Dogs"]),
        ("Mayonnaise", vec!["Sandwiches", "Burgers", "Coleslaw"]),
    ]);

    sauces_to_meals.insert("Mustard", vec!["Hot dog", "Burgers", "Pretzels"]);

    println!("Sauces (before): {:#?}", sauces_to_meals);
    match sauces_to_meals.remove("Mayonnaise") {
        Some(meals) => {
            println!("Meals: {:#?}", meals);
        }
        None => println!("Nothing to print here.")
    }
    println!("Sauces (after): {:#?}", sauces_to_meals);

    /*
     * Returns an Option<&Vec<&str>>.
     * The HashMap is still the owner of the vector.
     * We don't transfer ownership.
     * Rust wants its composite types to be the owner of what's contained inside.
     */
    let mustard_meals: Option<&Vec<&str>> = sauces_to_meals.get("Mustard");
    match mustard_meals {
        Some(meals) => {
            println!("Meals (Mustard): {:#?}", meals);
        }
        None => println!("Nothing to print here.")
    }

    sauces_to_meals.entry("Soy Sauce").or_insert(vec!["Sushi", "Dumplings"]);
    println!("Sauces (final): {:#?}", sauces_to_meals);
}
