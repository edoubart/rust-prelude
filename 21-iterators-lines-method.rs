/*
 * Cargo Crates
 */
use std::fs;
use std::io;

fn main() -> io::Result<()> {
    let contents: String = fs::read_to_string("story.txt")?;

    /*
     * The `.lines()` method returns an iterator of the string's individual
     * lines.
     */
    for line in contents.lines() {
        println!("{}", line);
    }

    Ok(())
}
