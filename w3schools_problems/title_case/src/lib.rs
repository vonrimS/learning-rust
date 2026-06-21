// Read a sentence from input (all lowercase, all not numeric).
// Capitalize the first letter of each word and print the result.
// Words are separated by single spaces.

use std::io;

pub fn read_and_format_input() -> String{
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("...cannot read your input");

        let trimmed = input.trim();

        if is_valid(&trimmed) {
            let cleaned: Vec<&str> = trimmed.split_whitespace().collect();
            break cleaned.join(" ");
        }

        println!("...invalid input. Try again:");
    }
}


pub fn is_valid(s: &str) -> bool{
    !s.is_empty() && s.chars().all(|c| c.is_alphabetic() || c.is_whitespace())
}


pub fn capitalize(s: &str) -> String{
    let mut chars = s.chars();

    match chars.next() {
        None => String::new(),
        Some(first) => first.to_uppercase().collect::<String>() + &chars.as_str().to_lowercase(),
    }
}

pub fn process(s: &str) -> String{
    let res = s.split(' ')
        .map(|w| capitalize(w) + " ")
        .collect::<String>()
        .trim_end()
        .to_string();

    res
}



#[cfg(test)]

mod tests{
    use super::*;

    #[test]
    fn test_proccess(){
        assert_eq!(process("one two three"), "One Two Three".to_string());
        assert_eq!(process("oNE tWO tHREE"), "One Two Three".to_string());
        assert_eq!(process("ONE TWO THREE"), "One Two Three".to_string());
    }

    #[test]
    fn test_is_valid_ok(){
        assert!(is_valid("one two three"));
        assert!(is_valid("onetwothree"));
    }

    #[test]
    fn test_is_valid_nok(){
        assert!(!is_valid("one two three!"));
        assert!(!is_valid(""));

        // it will be trimmed
        assert!(!is_valid("  ".trim()));
    }
}