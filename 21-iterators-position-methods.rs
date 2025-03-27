fn main() {
    let performers: [&str; 3] = [
        "Rustful Five",
        "Rust in Peace",
        "Rustin Bieber",
    ];

    /*
     * `.last()`
     */
    // `.into_iter()`
    let last: Option<&&str> = performers.into_iter().last();
    println!("{:?}", last);
    let last: &str = performers.into_iter().last().unwrap();
    println!("{}", last);

    // `.iter()`
    let last: Option<&&str> = performers.iter().last();
    println!("{:?}", last);
    let last: &&str = performers.iter().last().unwrap();
    println!("{}", last);

    /*
     * `.nth()`
     */
    // `.into_iter()`
    let second: Option<&&str> = performers.into_iter().nth(1);
    println!("{:?}", second);
    let second: &str = performers.into_iter().nth(1).unwrap();
    println!("{}", second);

    // `.iter()`
    let second: Option<&&str> = performers.iter().nth(1);
    println!("{:?}", second);
    let second: &&str = performers.iter().nth(1).unwrap();
    println!("{}", second);

    /*
     * `.nth_back()`
     */
    // `.into_iter()`
    let second_to_last: Option<&&str> = performers.into_iter().nth_back(1);
    println!("{:?}", second_to_last);
    let second_to_last: &str = performers.into_iter().nth_back(1).unwrap();
    println!("{}", second_to_last);

    // `.iter()`
    let second_to_last: Option<&&str> = performers.iter().nth_back(1);
    println!("{:?}", second_to_last);
    let second_to_last: &&str = performers.iter().nth_back(1).unwrap();
    println!("{}", second_to_last);

    /*
     * `.position()`
     */
    // `.into_iter()`
    let target_index: Option<usize> = performers
        .into_iter()
        .position(|element| {
            *element == "Rustin Bieber"
        });
    println!("{:?}", target_index);

    // `.iter()`
    let target_index: Option<usize> = performers
        .iter()
        .position(|element| {
            *element == "Rustin Bieber"
        });
    println!("{:?}", target_index);
}
