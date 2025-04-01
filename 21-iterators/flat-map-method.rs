fn main() {
    let attendees: [&str; 3] = [
        "Bob, Mary, Kevin",
        "Mike, Robbie, Matt, Austin",
        "Piers, Liam",
    ];

    let attendees: Vec<&str> = attendees
        .iter()
        .map(|group: &&str| { // First operation
            group.split(", ")
        })
        .flatten() // Second operation
        .collect();
    println!("{:?}", attendees);

    let attendees: Vec<&str> = attendees
        .iter()
        .flat_map(|group: &&str| { // Two operations in one iterator
            group.split(", ")
        })
        .collect();
    println!("{:?}", attendees);
}
