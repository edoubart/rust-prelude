fn main() {
    let numbers: Vec<i32> = vec![4, 8, 15, 16, 23, 42];

    /*
     * An **adapter method** is one that transforms an iterator into another
     * iterator based on some logic.
     */
    //let my_iterator = numbers.iter();

    /*
     * The `.map()` method applies a closure to each item in an iterator to
     * arrive at a new iterator of values.
     */
    //let squares = my_iterator.map(|number: &i32| {
    //    number.pow(2)
    //});

    // error[E0382]: borrow of moved value: `my_iterator`
    //println!("{:?}", my_iterator);
    //println!("{:?}", squares);

    //for number in squares {
    //    println!("Square: {}", number);
    //}

    // error[E0382]: borrow of moved value: `squares`
    //println!("{:?}", squares);


    /*
     * An **adapter method** is one that transforms an iterator into another
     * iterator based on some logic.
     */
    //let my_iterator = numbers.into_iter();

    /*
     * The `.map()` method applies a closure to each item in an iterator to
     * arrive at a new iterator of values.
     */
    //let squares = my_iterator.map(|number: i32| {
    //    number.pow(2)
    //});

    // error[E0382]: borrow of moved value: `my_iterator`
    //println!("{:?}", my_iterator);
    //println!("{:?}", squares);

    //for number in squares {
    //    println!("Square: {}", number);
    //}

    // error[E0382]: borrow of moved value: `squares`
    //println!("{:?}", squares);

    for number in numbers.into_iter().map(|number: i32| number.pow(2)) {
        println!("Square: {}", number);
    }
}
