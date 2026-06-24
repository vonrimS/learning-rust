// Read a string from input.
// Count the number of vowels: 
// (a, e, i, o, u). 
// Counting is case-insensitive (both "A" and "a" count).
// Print the result:
// Vowels: [count]

use std::io;

pub fn read_input() -> String{
    let mut input = String::new();
    loop{
        input.clear();
        io::stdin().read_line(&mut input).expect("...cannot read your input");
        let trimmed = input.trim().to_lowercase();

        if is_valid(&trimmed){
            break trimmed.to_string();
        }

        println!("...invalid input! Try again:");
    }
}


pub fn is_valid(s: &str) -> bool {
    !s.is_empty() && s.chars().all(|c| c.is_ascii_alphabetic() || c.is_ascii_whitespace())
}


pub fn count_vowels(s: &str) -> u32{
    let mut count = 0;
    let vowels = ['a', 'e', 'i', 'o', 'u'];
    
    for ch in s.chars(){
        if vowels.contains(&ch) {
            count += 1;
        }
    }
    count
}


#[cfg(test)]

mod tests{
    use super::*;

    #[test]
    fn test_is_valid(){
        assert!(is_valid("abc a"));
        assert!(is_valid(" "));
        assert!(is_valid("abc"));
        assert!(is_valid("abc"));
        
        assert!(!is_valid(""));
        assert!(!is_valid("abc1"));
    }
    
    #[test]
    fn test_count_vowels(){
        assert_eq!(count_vowels("abc"), 1);
        assert_eq!(count_vowels("hello world"), 3);
    }
}