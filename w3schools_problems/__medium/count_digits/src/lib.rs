// Read a positive number from input.
// Write a function that counts how many digits the number has.
// Print the result:
// Digits: [count]

use std::io;

pub fn read_input() -> String {
    println!("Enter your number:");
    let mut input = String::new();

    loop {
        input.clear();
        io::stdin()
            .read_line(&mut input)
            .expect("...cannot read your input");

        let trimmed = input.trim();
        if is_valid(trimmed) {
            break trimmed.to_owned();
        }

        println!("...invalid input! Try again:");
    }
}

pub fn is_valid(s: &str) -> bool {
    !s.is_empty() && s.chars().all(|c| c.is_ascii_digit())
}

pub fn count_digits(s: &str) -> usize {
    s.chars().count()
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn test_count_digits() {
        assert_eq!(count_digits("1"), 1);
        assert_eq!(count_digits("11"), 2);
        assert_eq!(count_digits("111"), 3);
    }

    #[test]
    fn test_is_valid() {
        assert!(is_valid("1"));
    }
}
