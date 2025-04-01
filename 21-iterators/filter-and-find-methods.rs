fn main() {
    let numbers: [i32; 7] = [10, 13, 23, 2, 8, 9, 6];

    /*
     * The `.filter()` method extracts a subset of values that satisfy a
     * condition. Pass a closure that returns true for the elements to keep and
     * false for the elements to exclude.
     * Returns all of the matches.
     */
    let evens: Vec<i32> = numbers
    //let evens: Vec<&i32> = numbers
    //let evens: Vec<i32> = numbers
        .iter()
        //.into_iter()
        .filter(|number: &&i32| *number % 2 == 0)
        //.filter(|number: &i32| number % 2 == 0)
        // Creates an iterator which copies all of its element.
        // This is useful when you have an iterator over `&T`, but you need an
        // iterator over `T`.
        .copied()
        .collect();
    println!("{:?}", evens);
    println!("{:?}", numbers);

    /*
     * The `.find()` method.
     * Returns the first match.
     */
    let first_even: Option<i32> = numbers
        .iter()
        //.into_iter()
        .find(|number: &&i32| *number % 2 == 0)
        .copied();
    println!("{:?}", first_even);

    let first_odd: Option<i32> = numbers
        .iter()
        //.into_iter()
        .find(|number: &&i32| *number % 2 != 0)
        .copied();
    println!("{:?}", first_odd);

    let nothing: Option<i32> = numbers
        .iter()
        //.into_iter()
        .find(|number: &&i32| **number > 100)
        .copied();
    println!("{:?}", nothing);

    let last_even: Option<i32> = numbers
        .iter()
        //.into_iter()
        .rfind(|number: &&i32| *number % 2 == 0)
        .copied();
    println!("{:?}", last_even);

    let last_odd: Option<i32> = numbers
        .iter()
        //.into_iter()
        .rfind(|number: &&i32| *number % 2 != 0)
        .copied();
    println!("{:?}", last_odd);
}
