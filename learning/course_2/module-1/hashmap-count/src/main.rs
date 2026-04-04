

use std::collections::HashMap;

use std::io;

fn logic(numbers: Vec<i32>) -> Vec<(i32, u32)> {

    let mut frequencies = HashMap::new();

    for num in numbers {
        let frequency = frequencies.entry(num).or_insert(0);
        *frequency += 1;
    }

    let mut result = Vec::new();

    for (num, frequency) in frequencies {
        result.push((num, frequency));
    }

    result
}


fn main() {
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 1, 3];
    let result = logic(numbers);
    //print the results in a human readable format that explains what the result is.
    println!(
        "The frequency of each number in the vector is: {:?}",
        result
    );

    let mut input = String::new();

    print!("Typing array of nums to calculate frequencies of each element: ");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let numbers: Vec<i32> = input
        .trim()
        .split_whitespace()
        .map(|s| s.parse().expect("Please enter valid integers"))
        .collect();

    let frequencies = logic(numbers);

    loop 
    {
        input.clear();
        print!("Typing elements you want to check the frequency of (or type 'exit' to quit): ");
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let input = input.trim();
        if input.eq_ignore_ascii_case("exit") {
            break;
        }

        let element: i32 = input.parse().expect("Please enter a valid integer");

        let frequency = frequencies.iter().find(|(num, _)| *num == element).map(|(_, freq)| *freq).unwrap_or(0);

        println!("The frequency of {} is: {}", element, frequency);
    }

    // access the frequency of a specific element in the result vector and print it in a human readable format.

    input.clear();

    let frequencies_map: HashMap<i32, u32> = frequencies.into_iter().collect();

    loop 
    {
        print!("Typing elements you want to check the frequency of (or type 'exit' to quit): ");
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let input = input.trim();
        if input.eq_ignore_ascii_case("exit") {
            break;
        }

        let element: i32 = input.parse().expect("Please enter a valid integer");

        let frequency = frequencies_map.get(&element).unwrap_or(&0);

        println!("The frequency of {} is: {}", element, frequency);
    }
    
}