use crate::config::{LogLevel, LogOutput, Logging};



pub fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}

pub fn farewell(name: &str) -> String {
    format!("Goodbye, {}!", name)
}

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

pub struct User {
    pub name: String,
    pub age: u32,
}


fn example() {
    let mut log = Logging::new(false, LogLevel::Info, LogOutput::Stdout);

    println!("{}", log.level());

    log.set_level(LogLevel::Debug);
}

pub mod parser {
    pub fn parse_config(config_str: &str) -> Result<(), String> {
        // Placeholder for actual parsing logic
        if config_str.is_empty() {
            Err("Config string cannot be empty".to_string())
        } else {
            Ok(())
        }
    }

    use regex::Regex;


    pub fn parse(){
        let re = Regex::new(r"^\w+$").unwrap();
        let test_str = "valid_config";
        assert!(re.is_match(test_str));
    }
}

pub fn ie_reverse_twice(s: String) -> String {
    s.chars().rev().collect::<String>().chars().rev().collect()
}

pub fn ie_add(a: i32, b: i32) -> i32 {
    a+b as i32
}



use proptest::prelude::*;

proptest! {

    #[test]
    fn reverse_twice_returns_original(input in ".*"){
        let result = ie_reverse_twice(input.clone());

        prop_assert_eq!(result, input);
    }

    #[test]
    fn addition_is_commutative(a in -1000i32..1000i32, b in -1000i32..1000i32) {
        prop_assert_eq!(ie_add(a , b), ie_add(b, a));
    }
}

// example with fuzz
pub fn parse_input(data: &[u8]) {
    let s = std::str::from_utf8(data).unwrap(); // dangerous
    println!("{}", s);
}


// bash: cargo fuzz run fuzz_target_1
