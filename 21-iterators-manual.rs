fn main() {
    let numbers: Vec<i32> = vec![4, 8, 15, 16, 23, 42];

    let mut current_index: usize = 0;
    let final_index: usize = numbers.len() - 1;

    /*
     * Option 1: `loop`:
     *   - Manual termination of iteration with `break` keyword;
     *   - Manual incrementing/decrementing of index;
     *   -> Risk of infinite loop.
     */
    //loop {
    //    if current_index > final_index {
    //        break;
    //    }

    //    println!("{}", numbers[current_index]);

    //    current_index += 1;
    //}

    /*
     * Option 2: `while`
     *   - No manual termination of iteration -> automatic;
     *   - Still manual incrementing/decrementing of index;
     *   -> Still risk of infinite loop.
     */
    //while current_index <= final_index {
    //    println!("{}", numbers[current_index]);

    //    current_index += 1;
    //}

    /*
     * Option 3: `for` (recommended, Rust standard)
     *   - No manual termination of iteration -> automatic;
     *   - No manual incrementing/decrementing of index -> automatic;
     *   -> No risk of infinite loop.
     */
    for number in numbers {
        println!("{}", number);
    }
}
