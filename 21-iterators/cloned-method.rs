fn main() {
    /*
     * The `.copied()` method converts an iterator from storing &T elements to T
     * elements. It makes a copy of each T element. The T data type must implement
     * the Copy trait.
     */

    /*
     * The `.cloned()` method similarly converts an iterator from storing &T
     * elements to T elements. It makes a clone of each T element. The T data type
     * must implement the Clone trait.
     * Primarly for Heap-based data (String, Vec, ...).
     */
    let teas: [String; 3] = [
        String::from("Hot Earl Grey"),
        String::from("Iced Green"),
        String::from("Hot Matcha"),
    ];

    // Iterator has String.
    //let more_teas: Vec<String> = teas
    //    .iter()
    //    .cloned() // &String -> String (double used space in memory)
    //    .collect();
    //// Iterator has &String.
    ////let more_teas: Vec<&String> = teas.iter().collect();
    //println!("{:?}", more_teas);

    //let more_teas: Vec<String> = teas
    //    .iter()
    //    .map(|tea: &String| tea.clone())
    //    .collect();
    //println!("{:?}", more_teas);

    //let more_teas: Vec<String> = teas
    //    .clone()
    //    .into_iter()
    //    //.iter()
    //    .collect();
    //println!("{:?}", more_teas);

    //let more_teas: Vec<String> = teas
    //    .iter()
    //    .filter(|tea: &&String| tea.contains("Hot"))
    //    .cloned()
    //    .collect();
    //println!("{:?}", more_teas);

    let more_teas: Vec<String> = teas
        .iter()
        .cloned() // Less efficient
        .filter(|tea: &String| tea.contains("Hot"))
        .collect();
    println!("{:?}", more_teas);
}
