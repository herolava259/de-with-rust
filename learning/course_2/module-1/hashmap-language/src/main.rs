
use std::collections::HashMap;

struct ProgrammingLanguage {
    name: String,
    year_created: i32,
    number_active_user: i32,
}

impl ProgrammingLanguage {
    fn new(name: String, year_created: i32, number_active_user: i32) -> Self {
        ProgrammingLanguage {
            name,
            year_created,
            number_active_user,
        }
    }

    pub fn calculate_weight(&self, min_year: i32, max_year: i32, total_active_user: i32) -> i32 {
        let normalized_year = 0.1 * (self.year_created - min_year) as f64 / (max_year - min_year) as f64 
                                    + 0.9 * (self.number_active_user as f64) / (total_active_user as f64);
        (normalized_year * 99.0) as i32 + 1
    }
}

fn take_input() -> ProgrammingLanguage
{
    println!("Enter the programming language name:");
    let mut name = String::new();
    std::io::stdin().read_line(&mut name).expect("Failed to read line");
    let name = name.trim().to_string(); 

    println!("Enter the year the programming language was created:");
    let mut year_input = String::new();
    std::io::stdin().read_line(&mut year_input).expect("Failed to read line");
    let year_created: i32 = year_input.trim().parse().expect("Please enter a valid number");

    println!("Enter the number of active users:");
    let mut users_input = String::new();
    std::io::stdin().read_line(&mut users_input).expect("Failed to read line");
    let number_active_user: i32 = users_input.trim().parse().expect("Please enter a valid number");

    ProgrammingLanguage {
        name,
        year_created,
        number_active_user,
    }
}

fn init_languages() -> HashMap<String, i32> {
    let mut languages = HashMap::new();

    languages.insert("JavaScript".to_string(), 1995);
    languages.insert("HTML/CSS".to_string(), 1990);
    languages.insert("Python".to_string(), 1991);
    languages.insert("SQL".to_string(), 1974);
    languages.insert("TypeScript".to_string(), 2012);
    languages.insert("Bash/Shell".to_string(), 1989);
    languages.insert("Java".to_string(), 1995);
    languages.insert("C#".to_string(), 2000);
    languages.insert("C++".to_string(), 1985);
    languages.insert("C".to_string(), 1972);
    languages.insert("PHP".to_string(), 1995);
    languages.insert("PowerShell".to_string(), 2006);
    languages.insert("Go".to_string(), 2007);
    languages.insert("Rust".to_string(), 2010);
    
    languages
}

fn calculate_weights(years_active: &mut HashMap<String, i32>) -> HashMap<String, i32> {

    for year in years_active.values_mut(){
        *year = 2024 - *year;
    }

    let min_year = *years_active.values().min().unwrap_or(&0);
    let max_year = *years_active.values().max().unwrap_or(&0);

    let mut weights = HashMap::new();

    for (language, &year) in years_active.iter() {
        let normalized_year = (year - min_year) as f64 / (max_year - min_year) as f64;
        let weight = (normalized_year * 99.0) as i32 +1 ;

        weights.insert(language.to_string(), weight);
    }

    weights
}

fn main() {
    let mut languages = init_languages();
    let weights = calculate_weights(&mut languages);
    
    println!("Language weighing from 1-100 by age (1 is newest and 100 is oldest):");
    for (language, weight) in &weights {
        println!("{}: {}", language, weight);
    }
}