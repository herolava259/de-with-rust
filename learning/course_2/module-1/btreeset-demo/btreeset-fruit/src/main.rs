use rand::seq::{SliceRandom};
use std::{collections::{BTreeSet, HashMap}};
use clap::{Parser, Subcommand};


#[derive(Parser, Debug)]
#[command(version, about, long_about=None)]
struct Cli
{
    #[arg(short, long, default_value_t=String::from("1, 3, 5, 7, 9"))]
    amounts: String,

    #[command(subcommand)]
    command: Commands
}

#[derive(Subcommand, Debug)]
enum Commands
{
    Remove {
        #[arg(short, long, default_value_t = String::from("apple"))]
        name: String
    },

    PrintOutUnique
    {
        #[arg(short, long, default_value_t = false)]
        reverse: bool
    },

    StatsFreq
    {
        #[arg(short, long, default_value_t = false)]
        list_all: bool,

        #[arg(short, long, default_value_t = String::from("apple, banana"))]
        names: String
    }
}

use std::time::Instant;

fn main() {
    let fruits = vec![
        "apple",
        "banana",
        "cherry",
        "date",
        "elderberry",
        "fig",
        "grape",
        "honeydew",
        "pineapple",
        "kiwi",
        "lemon",
        "mango",
        "nectarine",
        "orange",
        "papaya",
        "quince",
        "raspberry",
        "strawberry",
        "tangerine",
        "watermelon",
        "starfruit",
        "blueberry",
        "blackberry",
        "cantaloupe",
        "dragonfruit",
        "grapefruit",
        "jackfruit",
        "kumquat",
        "lychee",
        "mulberry",
        "olive",
        "peach",
        "pear",
        "plum",
        "pomegranate",
        "quararibea",
        "rambutan",
        "soursop",
        "tamarind",
        "ugli fruit",
    ];

    let cli = Cli::parse();



    let amounts: Vec<usize> = cli.amounts.split(", ")
                                              .map(|s| s.parse().expect("Input invalid inerger!"))
                                              .collect();

    let mut rng = rand::rng();

    let mut stats: HashMap<&str, usize> = HashMap::new();
    let mut global_set: BTreeSet<&str> = BTreeSet::new();

    for amount in amounts.iter() {
        let mut fruit_set = BTreeSet::new();
        let mut shuffled_fruits = fruits.clone();

        shuffled_fruits.shuffle(&mut rng);

        for fruit in shuffled_fruits {
            fruit_set.insert(fruit);
            *stats.entry(fruit).or_insert(0) += 1;

            global_set.insert(fruit);

            if fruit_set.len() >= *amount {
                break;
            }
        }

        println!("{}: {:?}", amount, fruit_set);
    }

    match cli.command
    {
        Commands::Remove { name } => {
            let start = Instant::now();
            println!("Remove the fruit {} at {:?}", name, start);
            if global_set.remove(name.as_str())
            {
                println!("Removing {} is successfully!", name);
            }
            else {
                println!("Cannot remove {} due to no existence in the set", name);
            }
            let elapsed = start.elapsed();
            println!("Took: {}ms to finish, with {} and total fruit in set is {}", elapsed.as_millis(), name, global_set.iter().len());

        },

        Commands::PrintOutUnique { reverse } => {
            //let unique_fruits: HashSet<&str> = stats.keys().copied().collect();

            if reverse
            {
                for fr in global_set.into_iter().rev()
                {
                    println!("Fruit {}", fr);
                }
            }
            else {
                for fr in global_set.into_iter()
                {
                    println!("Fruit {}", fr);
                }
            }
        },

        Commands::StatsFreq { list_all, names } => {
            if list_all
            {
                println!("Stats the frequency of each fruit");

                for (name, freq) in stats.iter()
                {
                    println!("Fruit {} appear {} times", *name, *freq);
                }
            }
            else {
                println!("Stats the fruits follow by the instruction");

                for name in names.split(", ")
                {
                    if !stats.contains_key(name)
                    {
                        println!("Fruit {} appear zero times", name);
                    }
                    else {
                        println!("Fruit {} appear {}", name, stats[name]);
                    }
                }
            }
        }
    }

}
