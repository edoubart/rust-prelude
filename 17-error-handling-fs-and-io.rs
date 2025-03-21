// Cargo Crates
use std::fs::File;
use std::io::{stdin, Error, Read};
use std::process;

fn main() {
    println!("Please enter the name of the file you'd like to read:");
    let mut input: String = String::new();

    let user_requested_file: Result<usize, Error> = stdin()
        .read_line(&mut input);

    if let Err(error) = user_requested_file {
        eprintln!(
            "Something went wrong collecting user input. The error was {:#?}",
            error
        );
        process::exit(1);
    }

    println!("input: {:#?}", input);

    let mut file: File = match File::open(input.trim()) {
        Ok(file) => file,
        Err(error) => {
            eprintln!(
                "Something went wrong opening the file. The error was {:#?}",
                error
            );
            process::exit(1);
        }
    };

    let mut file_contents: String = String::new();
    let read_operation: Result<usize, Error> = file
        .read_to_string(&mut file_contents);

    if let Err(error) = read_operation {
        eprintln!(
            "Something went wrong reading the file as a string. The error was {:#?}",
            error
        );
        process::exit(1);
    }

    println!("{file_contents}");
}
