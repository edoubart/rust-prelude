use std::error::Error;
use std::fmt::Display;
use std::fs;
//use std::io::Error as IoError;
//use std::num::ParseIntError;

#[derive(Debug)]
struct NumberIsUnimpressiveError;

impl Display for NumberIsUnimpressiveError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "That number is just too small")
    }
}

impl Error for NumberIsUnimpressiveError {}

//fn read_number_from_file(path: &str) -> Result<String, IoError> {
//fn read_number_from_file(path: &str) -> Result<i32, IoError> {
fn read_number_from_file(path: &str) -> Result<i32, Box<dyn Error>> {
    //let file_contents: Result<String, IoError> = fs::read_to_string(path);

    //let file_contents: String = fs::read_to_string(path)?;

    let file_contents: String = match fs::read_to_string(path) {
        Ok(content /*: String*/) => content,
        Err(error /*: Error*/) => return Err(Box::new(error)),
    };

    println!("{file_contents:#?}");

    //let parsed_number: i32 = file_contents.parse::<i32>()?;

    //Ok(file_contents)
    //Ok(parsed_number)

    let parsed_number: i32 = match file_contents.parse::<i32>() {
        Ok(number /*: i32*/) => {
            println!("Number: {}", number);

            number
        }

        Err(error /*: ParseIntError*/) => {
            println!("Error: {}", error);

            return Err(Box::new(error))
        }
    };

    if parsed_number < 100 {
        return Err(Box::new(NumberIsUnimpressiveError));
    }

    Ok(parsed_number)
}

fn main() {
    // let result: Result<i32, ParseIntError> = "abc".parse::<i32>();
    
    //match "abc".parse::<i32>() {
    //    Ok(number /*: i32*/) => println!("{number}"),
    //    Err(error /*: ParseIntError*/) => println!("My ParseIntError: {error:?}"),
    //}
    
    //match read_number_from_file(/*path:*/ "value.txt") {
    //match read_number_from_file(/*path:*/ "nonsense.txt") {
    match read_number_from_file(/*path:*/ "value.txt") {
        Ok(value/*: i32*/) => println!("The number is {value}"),
        Err(error/*: Box<dyn Error>*/) => println!("The error is {error}"),
    }
}
