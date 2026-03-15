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