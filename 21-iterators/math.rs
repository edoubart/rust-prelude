fn main() {
    /*
     * Integers:
     */
    let numbers: Vec<i32> = vec![4, 8, 15, 16, 23, 42];

    let total: i32 = numbers.iter().sum();
    println!("{}", total);

    let product: i32 = numbers.iter().product();
    println!("{}", product);

    let max: Option<&i32> = numbers.iter().max();
    println!("{:?}", max);
    let max: &i32 = numbers.iter().max().unwrap();
    println!("{}", max);

    let min: Option<&i32> = numbers.iter().min();
    println!("{:?}", min);
    let min: &i32 = numbers.iter().min().unwrap();
    println!("{}", min);

    let count: usize = numbers.iter().count();
    println!("{}", count);

    let invalid: f64 = 0.0 / 0.0;
    let numbers: Vec<f64> = vec![4.6, 8.8, 0.0 / 0.0, 6.2, f64::NAN];
    println!("{:?}", numbers);

    /*
     * Floats:
     */
    let total: f64 = numbers.iter().sum();
    println!("{}", total);

    let total: f64 = numbers
        .iter()
        .filter(|number: &&f64| !number.is_nan())
        .copied()
        .fold(0.0, |total, current| total + current);
    println!("{}", total);

    // error[E0277]: the trait bound `f64: Ord` is not satisfied
    //let max: Option<&f64> = numbers.iter().max();
    //println!("{:?}", max);
    //let max: &f64 = numbers.iter().max().unwrap();
    //println!("{}", max);

    let max: Option<f64> = numbers
        .iter()
        .filter(|number| !number.is_nan())
        .copied()
        .reduce(|accumulator, current| {
            // Returns the maximum of the two numbers, ignoring NaN.
            accumulator.max(current)
        });
    println!("{:?}", max);
    let max: Option<f64> = numbers
        .iter()
        .copied()
        .reduce(|accumulator, current| {
            // Returns the maximum of the two numbers, ignoring NaN.
            accumulator.max(current)
        });
    println!("{:?}", max);
}
