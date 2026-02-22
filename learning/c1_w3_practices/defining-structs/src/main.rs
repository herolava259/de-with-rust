#[derive(Debug)]
struct Person {
    first_name: String, 
    last_name: String,
    age: u8,
}

fn main() {
    println!("{:?}", Person {
        first_name: "Farrer".to_string(),
        last_name: String::from("Matsuo"),
        age: 26
    });
}
