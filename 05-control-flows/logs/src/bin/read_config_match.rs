/*
 * Match statements: Good for dealing with an error
 */

// Function that returns a Result
fn read_config_file() -> Result<String, Error> {
    fs::read_to_string("config.json")
}

fn get_config() -> String {
    let default_config = String::from(
        "{ enable_debug: true }"
    );

    match read_config_file() {
        // Case where file is read successfully
        Ok(config) => config,
        Err(_err) => {
            // Error! Does something beyond just logging the error
            println!("Config read err, using default");

            default_config
        }
    }
}
