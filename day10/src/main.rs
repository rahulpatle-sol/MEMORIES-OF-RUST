use std::fs::File;/// 
// This program reads the contents of a file and prints it to the console.
// It handles errors gracefully, returning early if the file cannot be opened or read.
// It uses the `Result` type to manage potential errors in file operations.
use std::io::{self, Read};
// The `read_file_contents` function attempts to read the contents of a file.``

fn read_file_contents(path: &str) -> Result<String, io::Error> {
    let mut file = File::open(path)?; // Returns early if file can't be opened
    let mut contents = String::new();
    file.read_to_string(&mut contents)?; // Returns early if reading fails
    Ok(contents)
}

fn main() {
    match read_file_contents("./data.txt") {
        Ok(data) => println!("File contents:\n{}", data),
        Err(e) => eprintln!("Error reading file: {}", e),
    }
}
