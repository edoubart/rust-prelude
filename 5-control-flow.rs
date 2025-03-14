fn main() {
    fn color_to_number(color: &str) -> i32 {
        let value = match color {
            "red" => 1,
            "green" => 2,
            "blue" => 3,
            _ => 0,
        };

        value
    }

    fn factorial_1(mut number: i32) -> i32 {
        let mut result: i32 = number;

        loop {
            if number == 1 {
                result *= 1;
                break;
            }

            result *= (number -1);
            number -= 1;
        };

        return result;
    }

    fn factorial_2(number: i32) -> i32 {
        if number == 1 {
            return 1;
        } else {
            return number * factorial_2(number - 1);
        }
    }

    println!("{}", factorial_1(5));
    println!("{}", factorial_1(4));

    println!("{}", factorial_2(5));
    println!("{}", factorial_2(4));
}
