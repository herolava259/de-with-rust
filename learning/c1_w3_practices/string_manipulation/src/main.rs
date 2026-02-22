
fn longest_word(sentence: &str) -> &str {
    let words = sentence.split_whitespace();
    let mut longest = "";

    for word in words {
        if word.len() > longest.len() {
            longest = word;
        }
    }

    longest
}

fn main() {
    let sentence = "the quick brown fox jumps over the lazy dog".to_string();

    println!("Sliced stenence 0-->4: {}", &sentence[0..=4]);

    let description = format!("Title: Quick story\n{}", sentence);

    println!("{}", description);

    let vowels = ['a', 'e', 'i', 'o', 'u'];  

    for c in sentence.chars() {
        match c {
            'a' | 'e' | 'i' | 'o' | 'u' => println!("{} is a vowel", c),
            _ => println!("{} is a consonant", c),
        }
    }

    let words = sentence.split(' ').collect::<Vec<&str>>();

    println!("Words: {:?}", words);

    let reversed = sentence.chars().rev().collect::<String>();

    println!("Reversed: {}", reversed); 

    // challenge quest 1: count the number of eac vowel in the sentence
    let mut count_vowel: i32  = 0;
    for c in sentence.chars() {
        if vowels.contains(&c) {
            count_vowel += 1;
        }
    }   

    println!("Number of vowel in the sentence: {}", count_vowel);

    let example_sentence = "Rust is a systems programming language that runs blazingly fast, prevents segfaults, and guarantees thread safety.".to_string();

    let longest = longest_word(&example_sentence);

    println!("Longest word: {}", longest);

}
