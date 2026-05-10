use rand::seq::SliceRandom;


pub fn create_fruit_salad(mut fruits: Vec<String>) -> Vec<String> {
    let mut rng = rand::rng();
    fruits.shuffle(&mut rng);

    fruits
}



pub fn display_fruit_salad(fruits: Vec<String>) {
    println!("Your fruit salad contains:");
    for fruit in fruits {
        println!("{}", fruit);
    }
}

pub fn csv_to_vec(csv: &str) -> Vec<String> {
    csv.split(',')
        .map(|s| s.trim().to_string())
        .collect()
}