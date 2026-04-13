


pub fn is_palindrome(s: String) -> bool {

    let mut left = 0;
    let mut right = s.len() - 1;


    while left < right {

        if s.chars().nth(left).unwrap() != s.chars().nth(right).unwrap() {
            return false;
        }
        left += 1;
        right -= 1;

    }

    return true;

}


pub fn occurences(s: String, c: char) -> u64 {

    let mut count = 0;

    for ch in s.chars() {
        if ch == c{
            count += 1;
        }
    }

    count

}


pub fn reverse_string(s: String) -> String {

    s.chars().rev().collect()
}