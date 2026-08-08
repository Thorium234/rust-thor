use std::fs::File;
use std::io::{self, Read};

fn read_file_contents() -> Result<String, io::Error> {
    let mut file = File::open("hello.txt")?; // if open fails, return Err
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;     // if read fails, return Err
    Ok(contents)
}
fn main() {
    match read_file_contents() {
        Ok(text) => println!("File contents: {}", text),
        Err(e) => println!("Error reading file: {}", e),
    }
}
