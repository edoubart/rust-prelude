fn main() {
    let first_names: [&str; 4] = ["Casey", "Robert", "Cargo", "Dan"];
    let last_names: [&str; 3] = ["Johnson", "Smith", "Rustman"];

    for (first_name, last_name) in first_names.iter().zip(last_names) {
        println!("{} {}", first_name, last_name);
    }

    let complete_names = first_names
        .iter()
        .zip(last_names)
        .map(|(first_name, last_name)| {
        //.map(|(first_name: &&str, last_name: &str)| {
            format!("{} {}", first_name, last_name)
        })
        .collect::<Vec<String>>();
    println!("{:?}", complete_names);

}
