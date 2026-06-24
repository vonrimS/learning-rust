// Read a sentence from input (all lowercase).
// Capitalize the first letter of each word and print the result.
// Words are separated by single spaces.

use std::io;

pub fn read_input() -> String{
    let mut input = String::new();
    loop {
        input.clear();
        io::stdin().read_line(&mut input).expect("...cannot read your input");
        let trimmed = input.trim();

        if is_valid(trimmed){
            break trimmed.to_lowercase();
        }
        println!("...invalid input! Try again:");
    }    
}

pub fn is_valid(s: &str) -> bool {
    !s.is_empty() && s.chars().all(|c| c.is_ascii_alphabetic() || c.is_ascii_whitespace())
}

pub fn capitalize_word(s: &str) -> String{
    let mut chars = s.chars();

    match chars.next() {
        None => String::new(),
        Some(first) => first.to_uppercase().collect::<String>() + chars.as_str()
    }
}

pub fn process_input(input: &str) -> String {
    input.split(' ')
        .map(|w| capitalize_word(w) + " ")
        .collect::<String>()
        .trim_end()
        .to_string()
}



#[cfg(test)]

mod tests{
    use super::*;


    #[test]
    fn test_is_valid(){
        assert!(is_valid("abc"));
        assert!(is_valid("a bc"));
        assert!(is_valid("a b  c"));
        
        assert!(!is_valid(""));
        assert!(!is_valid("abc1"));
    }

    #[test]
    fn test_capitalize_word(){
        assert_eq!(capitalize_word("one"), "One");
    }

    #[test]
    fn test_process_word(){
        assert_eq!(process_input("one two three"), "One Two Three");
        assert_eq!(process_input("one  two  three"), "One  Two  Three");
    }
}