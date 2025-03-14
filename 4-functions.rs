fn main() {
    fn apply_to_jobs(number: i32, title: &str) {
        println!("I'm applying to {number} {title} jobs.");
    }

    apply_to_jobs(35, "Rust Developer");

    fn is_even(number: i32) -> bool {
        let result: bool;

        if (number % 2) == 0 {
            result = true;
        } else {
            result = false;
        }

        return result;
    }

    println!("Result: {}", is_even(8));
    println!("Result: {}", is_even(9));

    fn alphabets(text: &str) -> (bool, bool) {
        (text.contains('a'), text.contains('z'))
    }

    println!("{:?}", alphabets("aardvark"));
    println!("{:?}", alphabets("zoology"));
    println!("{:?}", alphabets("zebra"));
}
