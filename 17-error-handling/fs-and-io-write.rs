// Cargo Crates
use std::fs;
use std::io;
use std::process;

fn main() {
    let file_result: Result<String, io::Error> = write_to_file();

    match file_result {
        Ok(filename) => println!("Successfully wrote to file {}.", filename),
        Err(error) => {
            eprintln!("There was an error: {:#?}", error);
            process::exit(1)
        }
    }
}

fn write_to_file() -> io::Result<String> {
    let input: io::Stdin = io::stdin();

    println!("What file would you like to write to?");
    let mut filename: String = String::new();
    input.read_line(&mut filename)?;
    filename = filename.trim().to_string();
    println!("Filename: {:#?}", filename);

    println!("What would you like to write to the file?");
    let mut text: String = String::new();
    input.read_line(&mut text)?;
    println!("Text: {:#?}", text);

    fs::write(&filename, text)?;

    Ok(filename)
}
