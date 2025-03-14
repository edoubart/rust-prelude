#[derive(Debug)]
struct File {
    name: String,
}

#[derive(Debug)]
struct Folder {
    name: String,
    contents: Vec<File>,
}

impl Folder {
    fn new(name: String) -> Self {
        Self {
            name,
            contents: Vec::new(), // or `vec![]`
        }
    }

    fn create_file(&mut self, name: String) {
        let file: File = File {
            name,
        };

        // Vector takes ownership of file.
        self.contents.push(file);
    }

    fn delete_file(&mut self, index: usize) -> File {
        /*
         * Removes and returns the element at position `index` within the
         * vector, shifting all elements after it to the left.
         */
        self.contents.remove(index)
    }

    fn get_file(&self, index: usize) -> Option<&File> {
        // Provides a reference to the element at the given `index`.
        self.contents.get(index)
    }
}

fn main() {
    let mut folder: Folder = Folder::new(String::from("/home"));

    folder.create_file(String::from(".bashrc"));
    folder.create_file(String::from(".gitconfig"));

    println!("Folder (before): {folder:#?}");

    folder.delete_file(1);

    println!("Folder (after): {folder:#?}");

    //let file: Option<&File> = folder.get_file(0);
    let file: Option<&File> = folder.get_file(1);
    match file {
        Some(file) => println!("File: {file:#?}"),
        None => println!("There was no file."),
    }
}
