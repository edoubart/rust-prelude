/*
 * Try Operator: Propagate errors when you just don't know how to handle them
 */

// Function that returns a Result
fn read_config_file() -> Result<String, Error> {
    fs::read_to_string("config.json")
}

fn get_config() -> Result<String, Error> {
    // Don't have any way to handle an Err, propagate it up
    let config = read_config_file()?;

    Ok(config)
}

// Err can be propagated to main, which will return (and print) it
fn main() -> Result<(), Error> {
    let config = get_config()?;

    println!("Got a config: {}", config);

    Ok(())
}
