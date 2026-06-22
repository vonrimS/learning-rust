// Read a year from input.
// Print Leap year or Not a leap year.
// A year is a leap year if:
//      It is divisible by 4 and
//      It is NOT divisible by 100, unless it is also divisible by 400


use std::io;

pub fn read_input() -> u32{
    println!("Enter your year:");

    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("...cannot read your input");

        let trimmed = input.trim();

        if is_valid(trimmed){
            break trimmed.parse::<u32>().unwrap();
        }

        println!("...invalid input. Try again:");
    }
}

pub fn is_valid(s: &str) -> bool{
    s.len() == 4 && s.parse::<u32>().is_ok()
}

pub fn is_leap(yyyy: u32) -> bool {
    yyyy % 4 == 0 && (yyyy % 100 != 0 || yyyy % 400 == 0)
}

pub fn print_leap(yyyy: u32){
    if is_leap(yyyy){
        println!("{}", "Leap year!");
    } else {
        println!("{}", "...not a leap year")
    }
}


#[cfg(test)]

mod tests{
    use super::*;

    #[test]
    fn test_is_leap(){
        assert!(is_leap(2024));
        assert!(is_leap(2020));
        assert!(is_leap(2016));
        assert!(is_leap(2012));
        assert!(is_leap(2008));

        assert!(!is_leap(2026));
        assert!(!is_leap(2025));
        assert!(!is_leap(2023));
        assert!(!is_leap(2001));
    }

    #[test]
    fn test_is_valid(){
        assert!(is_valid("2021"));

        assert!(!is_valid(""));
        assert!(!is_valid(" "));
        assert!(!is_valid("2021a"));
        assert!(!is_valid("20021"));
    }
}

