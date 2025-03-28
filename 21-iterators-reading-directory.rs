/*
 * The fs::read_dir() function returns a io::Result<ReadDir> enum.
 * The ReadDir struct implements the Iterator trait.
 * The iterator yields Result<DirEntry, Error> enums.
 * The DirEntry struct supports a 'path' method.
 * The fs::metadata function returns a Metadata struct.
 * The Metadata struct includes an 'is_file' method.
 * The fs::read_to_string function returns a io::Result<String>.
 */
use std::fs;
use std::io;
use std::path;
use std::process;

fn main() -> io::Result<()> {
    // Option 1:
    //let directory: fs::ReadDir = fs::read_dir("./").unwrap_or_else(|error| {
    //    eprintln!("Could not read directory: {}", error);
    //    process::exit(1);
    //});
    //
    //for entry_result in directory {
    //    match entry_result {
    //        Ok(entry) => {
    //            println!("{:?}", entry.path())
    //        }
    //        Err(error) => {
    //            eprintln!("Could not read entry: {}", error);
    //        }
    //    }
    //}

    // Option 2:
    //for entry_result in fs::read_dir("./")? {
    //    match entry_result {
    //        Ok(entry) => {
    //            println!("{:?}", entry.path())
    //        }
    //        Err(error) => {
    //            eprintln!("Could not read entry: {}", error);
    //        }
    //    }
    //}

    // Option 3:
    //for entry_result in fs::read_dir("./")? {
    //    if let Ok(entry) = entry_result {
    //        println!("{:?}", entry.path())
    //    }
    //}

    // Option 4:
    for entry_result in fs::read_dir("./")? {
        let entry: fs::DirEntry = entry_result?;
        //println!("{:?}", entry.path())

        let entry_name: path::PathBuf = entry.path();

        let metadata: fs::Metadata = fs::metadata(&entry_name)?;

        if metadata.is_file() {
            println!("{:?}\n----------", entry_name);
            let contents: String = fs::read_to_string(&entry_name)?;
            println!("{}", contents);
        }
    }

    Ok(())
}
