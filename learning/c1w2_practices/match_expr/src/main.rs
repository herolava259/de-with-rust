use std::io;

fn main() {

    println!("Please enter a greeting");

    let mut name = String::new();

    io::stdin().read_line(&mut name).expect("Failed to read input");

    match name.trim() {
        "Goodbye" =>  println!("Farewell!"),
        "Hello" => println!("Greetings!"),
        _ => println!("Unknown greeting."),
    }
}
