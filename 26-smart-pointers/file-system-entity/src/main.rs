#[derive(Debug)]
enum FileSystemEntity {
    File {
        name: String,
    },
    Folder {
        name: String,
        content: Vec<FileSystemEntity>, // A Vector is an example of a Smart Pointer.
    },
}

fn main() {
    let rust_file: FileSystemEntity = FileSystemEntity::File {
        name: String::from("my_rust_code.rs")
    };

    let python_file: FileSystemEntity = FileSystemEntity::File {
        name: String::from("my_python_code.py")
    };

    let code_folder: FileSystemEntity = FileSystemEntity::Folder {
        name: String::from("Code Stuff"),
        content: vec![rust_file, python_file]
    };

    let screenplay: FileSystemEntity = FileSystemEntity::File {
        name: String::from("My Screenplay.txt")
    };

    let all_documents: FileSystemEntity = FileSystemEntity::Folder {
        name: String::from("Documents"),
        content: vec![screenplay, code_folder]
    };

    println!("{all_documents:#?}");
}
