use rand::seq::IndexedRandom;

use std::cmp::Ord;

use std::collections::BinaryHeap;

use clap::{Parser, Subcommand};


#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli{

    #[command(subcommand)]
    command: Commands
}

#[derive(Subcommand)]
enum Commands {
    
    RemoveTop{
        #[arg(short, long)]
        remove: bool,node
    },

    PrintOutUniqueList{
        #[arg(short, long, default_value_t = false)]
        reverse: bool,

    },

    StatsFrequency{
        #[arg(short, long, default_value_t = false)]
        reverse: bool,
    }
    
}



#[derive(Eq, PartialEq, Clone)]
enum Fruit {
    Fig,
    Other(String),
}

impl Ord for Fruit {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (Fruit::Fig, Fruit::Fig) => std::cmp::Ordering::Equal,
            (Fruit::Fig, Fruit::Other(_)) => std::cmp::Ordering::Greater,
            (Fruit::Other(_), Fruit::Fig) => std::cmp::Ordering::Less,
            (Fruit::Other(_), Fruit::Other(_)) => std::cmp::Ordering::Equal,
        }
    }
}

impl PartialOrd for Fruit {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn generate_fruit_salad() -> BinaryHeap<Fruit> {
    let mut rng = rand::rng();

    let fruits = vec![
        "Apple", 
        "Orange", 
        "Pear", "Peach", 
        "Banana", 
        "Fig", 
        "Fig", 
        "Fig", 
        "Fig",
        "Grape",
        "Grape",
        "Grape",
        "Grape",
        "Grape",
        "Snake Fruit",
        "Dragon Fruit",
        "Kiwi",
        "Mango",
        "coconut",
        "Pineapple",
        "tomato",
    ];

    let mut fruit_salad = BinaryHeap::new();

    let mut figs_count = 0;

    while figs_count < 50 {
        let fruit = fruits.choose(&mut rng).unwrap();

        if *fruit == "Fig" {
            figs_count += 1;
            fruit_salad.push(Fruit::Fig);
        } else {
            fruit_salad.push(Fruit::Other(fruit.to_string()));
        }
    }

    fruit_salad
}

fn main() {
    let fruit_salad = generate_fruit_salad();

    let mut cli_salad = fruit_salad.clone();

    println!("Random Fruit Salad with two servings of Figs:");

    for fruit in fruit_salad.into_iter() {
        match fruit {
            Fruit::Fig => println!("Fig"),
            Fruit::Other(name) => println!("{}", name),
        }
    }

    let cli = Cli::parse();

    match cli.command {
        Commands::RemoveTop { remove } => {
            if remove {
                println!("Removing the top element from the binary heap...");
                // Code to remove the top element from the binary heap
                let salad = cli_salad.pop();

                match salad {
                    Some(fruit) => match fruit {
                        Fruit::Fig => println!("Removed: Fig"),
                        Fruit::Other(name) => println!("Removed: {}", name),
                    },
                    None => println!("The binary heap is empty, nothing to remove."),
                }
            } else {
                println!("No removal action taken.");

                println!("Peek at the top element of the binary heap...");
                let salad = cli_salad.peek();
                match salad {
                    Some(fruit) => match fruit {
                        Fruit::Fig => println!("Top element: Fig"),
                        Fruit::Other(name) => println!("Top element: {}", name),
                    },
                    None => println!("The binary heap is empty, nothing to peek."),
                }
            }
        },
        Commands::PrintOutUniqueList { reverse } => {
            println!("Printing out unique list of fruits...");
            // Code to print out unique list of fruits, optionally in reverse order

            if reverse {
                println!("Printing in reverse order...");
                // Code to print in reverse order

                let mut previous_fruit: Option<Fruit> = None;
                for fruit in cli_salad.into_sorted_vec().into_iter().rev() {

                    if previous_fruit.is_some() && previous_fruit.as_ref().unwrap() == &fruit {
                        continue; // Skip duplicate
                    }

                    previous_fruit = Some(fruit.clone());
                    
                    match fruit {
                        Fruit::Fig => println!("Fig"),
                        Fruit::Other(name) => println!("{}", name),
                    }

                    
                }
            } else {
                println!("Printing in normal order...");
                // Code to print in normal order

                let mut previous_fruit: Option<Fruit> = None;

                for fruit in cli_salad.into_sorted_vec() {
                    if previous_fruit.is_some() && previous_fruit.as_ref().unwrap() == &fruit {
                        continue; // Skip duplicate
                    }
                    previous_fruit = Some(fruit.clone());
                    match fruit {
                        Fruit::Fig => println!("Fig"),
                        Fruit::Other(name) => println!("{}", name),
                    }
                }
            }

        },
        Commands::StatsFrequency { reverse } => {
            println!("Calculating frequency statistics of fruits...");
            // Code to calculate and print frequency statistics of fruits, optionally in reverse order
            use std::collections::HashMap;
            
            let mut frequency: HashMap<String, usize> = HashMap::new();

            for fruit in cli_salad.into_iter() {
                let name = match fruit {
                    Fruit::Fig => "Fig".to_string(),
                    Fruit::Other(name) => name,
                };

                *frequency.entry(name).or_insert(0) += 1;
            }

            println!("Fruit Frequency Statistics:");
            let mut frequency_vec: Vec<(&String, &usize)> = frequency.iter().collect();

            if reverse {
                println!("Printing in reverse order...");
                frequency_vec.sort_by(|a, b| b.1.cmp(a.1));
            } else {
                println!("Printing in normal order...");
                frequency_vec.sort_by(|a, b| a.1.cmp(b.1));
            }

            for (fruit, count) in frequency_vec {
                println!("{}: {}", fruit, count);
            }

        },
    }
}
