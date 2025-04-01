/*
 * Cargo Crates
 */
use std::ops::RangeInclusive;

fn main() {
    let fifty_numbers: RangeInclusive<i32> = 1..=50;

    /*
     * `.take()`
     */
    //for number in fifty_numbers.take(15) {
    //    print!("{}/", number);
    //}

    /*
     * `.rev()`
     */
    //for number in fifty_numbers.rev().take(15) {
    //    print!("{}/", number);
    //}

    /*
     * `.skip()`
     */
    //for number in fifty_numbers.skip(5).take(15) {
    //    print!("{}/", number);
    //}

    //for number in fifty_numbers.take(15).skip(5) {
    //    print!("{}/", number);
    //}

    /*
     * `.step_by()`
     */
    //for number in fifty_numbers.skip(5).step_by(2) {
    //    print!("{}/", number);
    //}

    // error[E0382]: borrow of moved value: `fifty_numbers`
    // For all those example, we move ownership.
    //println!("{:?}", fifty_numbers);

    // `.clone()` allow us to keep the original owner.
    for number in fifty_numbers.clone().skip(5).step_by(2) {
        print!("{}/", number);
    }
    println!("{:?}", fifty_numbers);
}
