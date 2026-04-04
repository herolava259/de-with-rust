use rand::seq::{IndexedRandom, SliceRandom};
use std::io;

fn main() {
    let mut fruit = vec![
        "Orange",
        "Fig",
        "Pomegranate",
        "Cherry",
        "Apple",
        "Pear",
        "Peach",
    ];

    let mut salad: Vec<String> = Vec::new();

    let mut input = String::new();

    let _ = io::stdin().read_line(&mut input);

    let user_fruits: Vec<String> = input.split_whitespace().map(|s| s.to_string()).collect();

    salad.extend(user_fruits);

    let mut rng = rand::rng();


    let random_salad = salad.choose(&mut rng);

    if let Some(random_fruit) = random_salad
    {
        println!("Random fruit from your salad: {}", random_fruit)
    }
    else {
        println!("Salad is empty!");
    }

    fruit.shuffle(&mut rng);

    println!("Fruit Salad:");

    for (i, item) in fruit.iter().enumerate() {
        if i != fruit.len() - 1 {
            print!("{}, ", item);
        } else {
            println!("{}", item);
        }
    }

    println!("How many random fruits to add?");

    let mut num_input = String::new();

    io::stdin().read_line(&mut num_input).unwrap();

    let n: usize = num_input.trim().parse().unwrap_or(0);

    for _ in 0..n {
        if let Some(fruit) = fruit.choose(&mut rng)
        {
            salad.push(fruit.to_string());
        }
    }

    println!("Final fruit salad: {:?}", salad);

}
