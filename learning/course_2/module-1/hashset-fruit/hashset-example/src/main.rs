use rand::seq::IndexedRandom;
use std::{collections::{HashMap, HashSet}};

use clap::Parser;


#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {

    #[arg(short, long, default_value_t = 10)]
    num_fruits: usize,

}




fn generate_fruit() -> &'static str {
    let fruits = [
        "Apple",
        "Banana",
        "Cherry",
        "Date",
        "Elderberry",
        "Fig",
        "Grape",
        "Honeydew",
    ];

    let mut rng = rand::rng();
    fruits.choose(&mut rng).unwrap()
}



fn main() {

    let args = Args::parse();
    let mut fruit_set = HashSet::new();

    let mut frequencies = HashMap::new();

    let mut unique_fruits = HashSet::new();

    println!("Generating {} random fruits...", args.num_fruits);

    for _ in 0..args.num_fruits {

        let fruit = generate_fruit();

        fruit_set.insert(fruit);

        let frequency = frequencies.entry(fruit).or_insert(0);

        *frequency += 1;

        if !unique_fruits.contains(fruit)
        {
            unique_fruits.insert(fruit);
        }
        else {
            unique_fruits.remove(fruit);
        }
    }

    println!("Number of unique fruits generated: {}", fruit_set.len());

    println!("Frequencies of each fruit:");

    for (fruit, frequency) in &frequencies {
        println!("{}: {}", fruit, frequency);
    }

    println!("Unique fruits (appeared exactly once):");
    for fruit in &unique_fruits {
        println!("{}", fruit);
    }
}
