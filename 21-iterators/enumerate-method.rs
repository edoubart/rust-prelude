fn main() {
    let applicants: Vec<&str> = vec![
        "Rob",
        "Bob",
        "Cob",
        "Alex",
        "Piers",
        "John",
        "Dan",
    ];

    /*
     * The `.enumerate()` adapter transforms an iterator so that it yields the
     * index position along with the current element.
     */
    //let winners = applicants
    //    .into_iter()
    //    .enumerate() // Returns a Tuple with index and captured element.
    //    .filter(|(index, _)| {
    //    //.filter(|(index: &usize, _applicant: &&str)| {
    //        index % 3 == 0
    //    })
    //    .map(|(_, applicant)| {
    //    //.map(|(index: usize, applicant: &str)| {
    //        applicant
    //    })
    //    .collect::<Vec<&str>>();
    //println!("{:?}", winners);

    let winners = applicants
        .into_iter()
        .enumerate() // Returns a Tuple with index and captured element.
        .filter_map(|(index, applicant)| { // Returns an Option Enum.
        //.filter(|(index: &usize, _applicant: &&str)| {
        //.map(|(index: usize, applicant: &str)| {
            if index % 3 == 0 {
                Some(applicant)
            } else {
                None
            }
        })
        .collect::<Vec<&str>>();
    println!("{:?}", winners);
}
