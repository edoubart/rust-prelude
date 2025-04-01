fn main() {
    let numbers: [i32; 6] = [4, 8, 15, 16, 23, 42];

    /*
     * The `.partition()` method groups and returns the values for which the
     * closure returns true and for which the closure returns false.
     */
    let (evens, odds): (Vec<i32>, Vec<i32>) = numbers
    //let groups: (Vec<i32>, Vec<i32>) = numbers
        .into_iter()
        .partition(|number: &&i32| { // Returns a Tuple with the two groups.
            *number % 2 == 0
        });
    println!("{:?}", evens);
    println!("{:?}", odds);
    //println!("{:?}", groups);
}
