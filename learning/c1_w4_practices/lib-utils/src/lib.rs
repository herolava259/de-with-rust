use std::io::{BufReader, BufRead};

pub mod colors;
pub mod example;
pub mod config;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

pub fn read_stdin() -> String {
    let stdin = std::io::stdin();

    let mut reader = BufReader::new(stdin.lock());

    let mut line = String::new();

    reader.read_line(&mut line).expect("Failed to read input line");

    line.trim().to_string()
}


fn _read_stdin<R: BufRead>(reader: &mut R) -> String {
    let mut line = String::new();

    reader.read_line(&mut line).expect("Failed to read input line");

    line.trim().to_string()
}


pub mod extensions_module {
    pub fn example_extension() -> String {
        "This is an example extension".to_string()
    }

    pub struct Name {
        pub first_name: String,
        pub last_name: String,
    }

    impl Name {
        pub fn full_name(&self) -> String {
            format!("{} {}", self.first_name, self.last_name)
        }

        pub fn new(first_name: &str, last_name: &str) -> Self {
            Self {
                first_name: first_name.to_string(),
                last_name: last_name.to_string(),
            }
        }

        fn private_method(&self) -> String {
            "This is a private method".to_string()
        }
    }
}


#[cfg(test)]
mod internal_tests {
    use std::io::Cursor;

    use super::_read_stdin;

    #[test]
    fn test_read_stdin() {
        let input = "Hello, world!\n";
        let mut reader = Cursor::new(input);

        let result = _read_stdin(&mut reader);

        assert_eq!(result, "Hello, world!");
    }

    #[test]
   fn test_read_input_empty() {
       let input = "";
       let expected_output = "";
       let mut reader = Cursor::new(input);
       let output = _read_stdin(&mut reader);
       assert_eq!(output, expected_output);
   }
}