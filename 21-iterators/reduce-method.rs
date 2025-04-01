fn main() {
    let earnings: [i32; 0] = [];
    //let earnings: [i32; 4] = [4, 7, 9, 13];

    /*
     * The `.reduce()` method is similar to `.fold()` but it supplies the
     * iterator's first element as the starter value. Therefore, it only accepts
     * a closure as parameter.
     * Also, the `.reduce()` method returns an **Option** enum to account for
     * the possibility of an empty iterator.
     */
    //let sum: Option<i32> = earnings
    //    .into_iter()
    //    .reduce(|total, current| {
    //        println!("Total: {}, current: {}", total, current);
    //        total + current
    //    });
    //println!("{:?}", sum);

    let address_portions: [String; 3] = [
        String::from("123 Elm Street"),
        String::from("Suburbia"),
        String::from("New Jersey"),
    ];
    println!("{}", address_portions.join(", "));

    let address: Option<String> = address_portions
        .into_iter()
        .reduce(|mut accumulator: &mut String, portion: &String| {
            accumulator.push_str(", ");
            accumulator.push_str(&portion);
            accumulator
        });
    println!("{:?}", address);
}
