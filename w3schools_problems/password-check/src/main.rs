use std::io;

// Read a password from input.
// If the password has 8 or more characters, print:
//  Valid
// Otherwise, print:
//  Invalid

fn main() {
    println!("Welcome to Password check App!");
    let password = read_input("Enter your password:");

    if is_valid(&password){
        println!("Valid");
    } else {
        println!("Invalid");
    }
}

fn read_input(s: &str) -> String{
    println!("{s}");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("...cannot read your input");
    input.trim().to_string()
}


fn is_valid(s: &str) -> bool {
    s.len() >= 8
}




#[cfg(test)]

mod tests{
    use super::*;

    #[test]
    fn is_valid_ok(){
        assert!(is_valid("12345678"));
    }

    #[test]
    fn is_valid_nok(){
        assert!(!is_valid("1234567"));
        assert!(!is_valid(""));
    }
}