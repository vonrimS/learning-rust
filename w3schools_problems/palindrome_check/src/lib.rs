// Read a word from input (all lowercase).
// Print "It's a palindrome" if the word is a palindrome (reads the same forwards and backwards).
// Print "...not a palindrome" if it is not.

use std::io;

pub fn read_input() -> String {
    println!("Enter your palindrome candidate here, only alphabetic:");
    let mut input = String::new();
    loop {
        input.clear();
        io::stdin().read_line(&mut input).expect("...cannot read your input");
        let trimmed = input.trim();
        if is_valid(&trimmed) {
            break trimmed.to_string();
        }

        println!("...invalid input! Try again:");
    }
}

pub fn is_valid(w: &str) -> bool{
    !w.is_empty() && w.chars().all(|c| c.is_ascii_alphabetic() || c.is_ascii_whitespace())
}

pub fn is_palindrome(w: &str) -> bool{
    w.chars().eq(w.chars().rev())
}

pub fn reverse_string(w: &str) -> String{
    w.chars().rev().collect()
}




#[cfg(test)]

mod tests{
    use super::*;

    #[test]
    fn test_reverse_string(){
        assert_eq!(reverse_string("abc"), "cba");
        assert_eq!(reverse_string("wow"), "wow");
        assert_eq!(reverse_string("ab "), " ba");
        assert_eq!(reverse_string(" "), " ");
    }

    #[test]
    fn test_is_palindrome(){
        assert!(is_palindrome("wow"));
        assert!(is_palindrome("abcba"));
        assert!(is_palindrome("abc cba"));

        assert!(!is_palindrome("a cba"));
        assert!(!is_palindrome("acba"));
        assert!(!is_palindrome("acca "));
    }
}