/*
 * Cargo Crates
 */
use std::env;

fn main() {
    /*
     * **Command-line arguments** are values passed in to the program from the
     * Terminal/command prompt.
     */
    let args: env::Args = env::args();

    for arg in args {
        println!("{}", arg);
    }
}
