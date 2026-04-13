use clap::{Parser};
use std::fs::read_to_string;
use std::path::Path;
use std::collections::HashMap;


#[derive(Parser)]
#[command(version = "1.0", author = "Tung",about = "A simple command-line tool to count the number of words in a file.")]
struct Args {
    /// The path to the file to be processed
    #[arg(short, long)]
    file: String,
}

fn main() {
    let args = Args::parse();

    let path = Path::new(&args.file);
    if !path.exists() 
    {
        eprintln!("Error: The file '{}' does not exist.", args.file);
        std::process::exit(1);
    }

    let mut counter = HashMap::new();

    for line in read_to_string(path).unwrap().lines() {
        for word in line.split_whitespace() {
            let count = counter.entry(word.to_string()).or_insert(0);
            *count += 1;
        }
    }

    println!("------------------Wordcount------------------");
    println!("|{:^10}: {:^10}|", "Word", "Count");
    for (word, count) in counter.iter() {
        println!("|{:^10}: {:^10}|", word, count);    
    }
    println!("---------------------------------------------");
}