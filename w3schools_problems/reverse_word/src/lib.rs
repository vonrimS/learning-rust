// Read a sentence from input.
// Print the words in reverse order.
// The words should be separated by spaces, just like the input.

// Input used in test:
//      I love coding

// Expected Output:
//      coding love I

use std::io;

pub fn read_input() -> String{
    println!("Type your sentence:");
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
    s.len() > 0 && s.chars().all(|c| c.is_ascii_alphabetic() || c.is_ascii_whitespace())
}

pub fn reverse_words(s: &str) -> String {
    let mut tokens: Vec<&str> = s.split(' ').collect();
    tokens.reverse();
    tokens.join(" ")
}



#[cfg(test)]

mod tests{
    use crate::reverse_words;


    #[test]
    fn test_reverse_words(){
        assert_eq!(reverse_words(" "), " ".to_string());
        assert_eq!(reverse_words(" one"), "one ".to_string());
        assert_eq!(reverse_words("one two"), "two one".to_string());
        assert_eq!(reverse_words("one two  three"), "three  two one".to_string());
        assert_eq!(reverse_words("One Two  Three"), "Three  Two One".to_string());
    }
}