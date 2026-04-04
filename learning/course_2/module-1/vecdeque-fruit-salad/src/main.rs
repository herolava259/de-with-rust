use rand::seq::{IndexedRandom, SliceRandom};
use std::collections::{VecDeque};
use std::{io};

fn main() {
    let mut fruit_deque: VecDeque<String> = VecDeque::new();

    fruit_deque.push_back("Arbutus".to_string());
    fruit_deque.push_back("Loquat".to_string());
    fruit_deque.push_back("Strawberry Tree Berry".to_string());

    let mut rng = rand::rng();

    let mut fruit_vec: Vec<_> = fruit_deque.into_iter().collect();

    fruit_vec.shuffle(&mut rng);

    let mut fruit_deque: VecDeque<_> = fruit_vec.into_iter().collect();

    fruit_deque.push_front("Pomegranate".to_string());
    fruit_deque.push_front("Fig".to_string());
    fruit_deque.push_back("Cherry".to_string());

    println!("Fruit Salad");

    for (i, item) in fruit_deque.iter().enumerate() {
        if i != fruit_deque.len() - 1 {
            print!("{}, ", item);
        } else {
            println!("{}", item);
        }
    }

    let mut input = String::new();

    loop {
        input.clear();

        print!("Add fruit to end of queue: ");
        io::Write::flush(&mut io::stdout()).unwrap();

        io::stdin().read_line(&mut input).unwrap();

        let fruit = input.trim().to_string();

        if fruit.is_empty() {
            break;
        }

        fruit_deque.push_back(fruit);
    }

    let vec_seq: Vec<_> = fruit_deque.into_iter().collect();

    println!("Choose random fruit from fruit tray: ");

    let mut rng = rand::rng();

    if let Some(fruit) = vec_seq.choose(&mut rng) {
        println!("Choosen fruit randomly: {}", fruit);
    }

    let mut num_rmv = String::new();

    print!("Can you type num of fruit you can remove from top of queue: ");
    io::Write::flush(&mut io::stdout()).unwrap();

    io::stdin().read_line(&mut num_rmv).unwrap();

    let n: usize = num_rmv.trim().parse().unwrap_or(0);

    let mut fruit_deque: VecDeque<_> = vec_seq.into_iter().collect();

    for _ in 0..n {
        if let Some(fruit) = fruit_deque.pop_front() {
            println!("Fruit is popped from top of queue: {}", fruit);
        } else {
            break;
        }
    }
}
