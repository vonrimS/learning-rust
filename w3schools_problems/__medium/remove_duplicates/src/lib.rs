// Read a string from input.
// Remove all consecutive duplicate characters, keeping only the first of each group.
// Print the result.

use std::{collections::HashSet, io};

pub fn read_input() -> String {
    let mut input = String::new();
    loop {
        input.clear();

        io::stdin().read_line(&mut input).expect("...cannot read your input");
        let trimmed = input.trim();

        if is_valid(trimmed){
            break trimmed.to_lowercase().to_string();
        }

        println!("...invalid input! Try again:");
    }
}

pub fn is_valid(s: &str) -> bool{
    !s.is_empty() && s.chars().all(|ch| ch.is_ascii_alphabetic() || ch.is_ascii_whitespace())
}

pub fn clear_duplicates(s: &str) -> String{
    let mut seen = HashSet::new();

    s.chars()
        .filter(|&c| seen.insert(c))
        .collect()
}


#[cfg(test)]

mod tests{
    use super::*;

    #[test]
    fn test_is_valid(){
        assert!(is_valid("abc"));
        assert!(is_valid("ab c"));
        assert!(is_valid("a  b c"));

        assert!(!is_valid(""));
        assert!(!is_valid("abc1"));
    }

    #[test]
    fn test_clear_dupblicates(){
        assert_eq!(clear_duplicates("abc"), "abc");
        assert_eq!(clear_duplicates("aabbcc"), "abc");
        assert_eq!(clear_duplicates("abcabc"), "abc");
        assert_eq!(clear_duplicates("ab c  def"), "ab cdef");
    }
}