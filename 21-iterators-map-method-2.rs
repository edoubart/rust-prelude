fn main() {
    let names: [String; 3] = [
        String::from("Jimmy"),
        String::from("Cleveland"),
        String::from("Boris"),
    ];

    // In Rust, we call the following a pipeline of Adapter Methods:
    let name_lengths = names
    //let name_lengths Vec<usize> = names
        .iter()
        .map(|name: &String| name.to_lowercase())
        .map(|name: String| name.replace("i", "@@"))
        .map(|name: String| name.len())
        .collect::<Vec<usize>>();
        //.collect();

    println!("{:?}", name_lengths);
}
