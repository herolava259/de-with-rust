//Construct a library that simplifies file input/output operations. Include functions to read a file and return its contents as a string, write a string to a file, and append content to an existing file.

use std::fs::{File, OpenOptions};
use std::io::{self, Read, Write};

pub fn read_file(file_path: &str) -> io::Result<String> {
    let mut file = File::open(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

pub fn write_to_file(file_path: &str, content: &str) -> io::Result<()> {
    let mut file = File::create(file_path)?;
    file.write_all(content.as_bytes())?;
    Ok(())
}

pub fn append_to_file(file_path: &str, content: &str) -> io::Result<()> {
    let mut file = OpenOptions::new().append(true).open(file_path)?;
    file.write_all(content.as_bytes())?;
    Ok(())
}

