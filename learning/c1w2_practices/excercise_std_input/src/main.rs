use std::io;

fn main() {
    println!("Please enter a greeting:");
    let mut name = String::new();
    io::stdin().read_line(&mut name).expect("Failed to read input");
    
    

    if name.trim().is_empty() {
        println!("I can't find a greeting, good bye.");
        return;
    }

    name = name.trim().to_string();

    let mut standard_name = String::new();

    let mut is_prev_space: bool = true;

    for c in name.chars() {
        if c == ' '{
            is_prev_space = true;
            standard_name.push(c);
        }
        else if c.is_alphabetic()
        {
            if is_prev_space{
                standard_name.push(c.to_ascii_uppercase());
            }
            else if c.is_uppercase() 
            {
                standard_name.push(c.to_ascii_lowercase());
            }
            else 
            {
                standard_name.push(c);
            }

            is_prev_space = false;
        }
        else 
        {
            standard_name.push(c);
            is_prev_space = false;
        }
    }

    println!("Standard Name {}", standard_name);

    // use of match expression to pattern match against variable "name"
    match standard_name.trim() {
        "Good Bye" => println!("Sorry to see you go."),
        "Hello" => println!("Hi, nice to meet you!"),
        _ => println!("I can't find a greeting, good bye."),
    }
}

