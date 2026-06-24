// Read a sentence from input.
// Find the longest word and print it.
// If two words have the same length, print the one that appears first.
// Words are separated by single spaces.

use std::io;

pub fn read_input() -> String{
    let mut input = String::new();

    loop {
        input.clear();

        io::stdin().read_line(&mut input).expect("...cannot read your input");
        let trimmed = input.trim();

        if is_valid(trimmed) {
            break trimmed.to_string();
        }

        println!("...invalid input. Try again:");
    }
}


pub fn is_valid(s: &str) -> bool{
    !s.is_empty() && s.chars().all(|c| c.is_ascii_alphabetic() || c.is_ascii_whitespace())
}

pub fn find_longest_word(s: &str) -> &str {
    let mut longest = "";

    for w in s.split_whitespace() {
        if w.len() > longest.len() {
            longest = w
        }
    }

    longest
}




#[cfg(test)]

mod tests{
    use super::*;

    #[test]
    fn test_find_longest_word(){
        assert_eq!(find_longest_word("a bb ccc"), "ccc");
        assert_eq!(find_longest_word("a bb cc"), "bb");
        assert_eq!(find_longest_word("a b c"), "a");
        assert_eq!(find_longest_word("a  bbb  cc  ddd"), "bbb");
    }
}