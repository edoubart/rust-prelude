// Cargo Crates
use std::fs::File;
use std::io::{self, stdin, Read};
use std::process;

fn main() {
    let file_result: Result<String, io::Error> = read_file();

    match file_result {
        Ok(contents) => println!("{contents}"),
        Err(error) => {
            eprintln!("There was an error: {:#?}", error);
            //process::exit(1)
        }
    }
}

fn read_file() -> Result<String, io::Error> {
    println!("Please enter the name of the file you'd like to read:");

    let mut input: String = String::new();

    stdin().read_line(&mut input)?;
    //let user_requested_file: Result<usize, io::Error> = stdin()
    //    .read_line(&mut input);

    //if let Err(error) = user_requested_file {
    //    return Err(error); // Equivalent to Result::Err(error)
    //    //eprintln!(
    //    //    "Something went wrong collecting user input. The error was {:#?}",
    //    //    error
    //    //);
    //    //process::exit(1)
    //}

    let mut file_contents: String = String::new();
    // Leverage the ? Operator (The Try Operator), and chain operations.
    File::open(input.trim())? // -> Returns a Result<T>
        .read_to_string(&mut file_contents)?; // -> Returns a Result<T>

    //let mut file: File = File::open(input.trim())?;
    //let mut file: File = match File::open(input.trim()) {
    //    Ok(file) => file,
    //    Err(error) => {
    //        return Err(error); // Equivalent to Result::Err(error)
    //        //eprintln!(
    //        //    "Something went wrong opening the file. The error was {:#?}",
    //        //    error
    //        //);
    //        //process::exit(1)
    //    }
    //};

    //let mut file_contents: String = String::new();
    //file.read_to_string(&mut file_contents)?;
    //let read_operation: Result<usize, io::Error> = file
    //    .read_to_string(&mut file_contents);

    //if let Err(error) = read_operation {
    //    return Err(error); // Equivalent to Result::Err(error)
    //    //eprintln!(
    //    //    "Something went wrong reading the file as a string. The error was {:#?}",
    //    //    error
    //    //);
    //    //process::exit(1)
    //}

    Ok(file_contents)
    //println!("{file_contents}");
}
