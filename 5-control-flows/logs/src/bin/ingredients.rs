use std::io::Error;

fn validate_ingredients(ingredients: &Vec<String>) -> Result<(), Error> {
    if ingredients.len() > 3 {
        Err(Error::other("'ingredients' needs to have a maximum of 3 elements"))
    } else {
        Ok(())
    }
}

fn main() {
    let ingredients = vec![
        String::from("Cheese"),
        String::from("Tomatoes"),
        String::from("Peppers"),
        //String::from("Olives"),
    ];

    match validate_ingredients(&ingredients) {
        Ok(..) => println!("Success!"),
        Err(error) => println!("Error: {}", error)
    }
}
