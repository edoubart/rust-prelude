fn main() {
    let spreadsheet: Vec<[i32; 3]> = vec![
        [100, 200, 300],
        [123, 456, 789],
        [987, 654, 321],
    ];

    let values: Vec<i32> = spreadsheet
        .into_iter()
        .flatten()
        .collect();
    println!("{:?}", values);
}
