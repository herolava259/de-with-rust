use std::{fs::File, io::{BufRead, BufReader}};

fn main() {
    let file = File::open("non_existent_file.txt");

    let file = match file {
        Ok(file) => file,
        Err(error) => {
            match error.kind() {
                std::io::ErrorKind::NotFound => {
                    panic!("File not found: {}. Please ensure the file exists.", error);
                },
                _ => {
                panic!("Error opening file: {}", error)
                }   
            }
        }
    };

    let reader = BufReader::new(file);

    for line in reader.lines() {
        println!("{}", line.unwrap());
    }


}
