use rand::seq::SliceRandom;
use std::{collections::HashSet, hash::Hash};

pub fn create_fruit_salad(num_fruits: usize) -> Vec<String> {

    let fruits = vec![
        "Arbutus".to_string(),
        "Loquat".to_string(),
        "Strawberry Tree Berry".to_string(),
        "Pomegranate".to_string(),
        "Fig".to_string(),
        "Cherry".to_string(),
        "Orange".to_string(),
        "Pear".to_string(),
        "Peach".to_string(),
        "Apple".to_string(),
    ];

    let mut rng = rand::rng();

    let mut fruits = fruits;

    fruits.shuffle(&mut rng);

    fruits.into_iter().take(num_fruits).collect()
}


pub fn validate_inputs(fruits: &Vec<String>) -> bool {

    let mut fruit_set: HashSet<String> = HashSet::new();

    for fruit in fruits {
        if fruit_set.contains(fruit) {
            return false;
        }
        fruit_set.insert(fruit.to_string());
    }
    true
}



pub fn create_fruit_salad_with_fruit_options(num_fruits: usize, optional_fruits: &mut Vec<String>) -> Vec<String>
{
    let num_imparative_fruit = num_fruits - optional_fruits.len();

    let mut imperative_fruits = create_fruit_salad(num_imparative_fruit);

    imperative_fruits.extend(optional_fruits.iter().cloned());

    imperative_fruits.sort();

    imperative_fruits
}