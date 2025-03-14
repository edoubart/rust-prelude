fn main() {
    // By copy
    let is_concert: bool = false;
    let is_event: bool = is_concert;
    // No, Rust will not move ownership because `is_concert` is of type Boolean
    // which is a Scalar Type/Primitive and implement the 'Copy' trait.
    // Let's try it out!
    println!("is_concert: {is_concert}");
    println!("is_event: {is_event}");

    // By copy
    // str -> "String Literal" or "String Slice"
    let sushi: &str = "Salmon";
    let dinner: &str = sushi;
    // No, Rust will not move ownership.
    drop(sushi);
    //println!("sushi: {sushi}");
    println!("dinner: {dinner}");

    // By reference
    // String -> "Heap String"
    let sushi: String = String::from("Salmon");
    let dinner: &String = &sushi;
    // Yes, Rust will move ownership.
    println!("sushi: {sushi}");
    println!("dinner: {dinner}");

    fn eat_meal(mut meal: String) {
        meal.clear()
    }

    let mut meal: String = String::from("fish papas and salad");
    println!("meal before: {}", meal);
    eat_meal(&mut meal);
    println!("meal after: {meal}");
}
