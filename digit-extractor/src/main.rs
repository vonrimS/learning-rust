use std::io;

// Read a 3-digit number from input.
// Print each digit on its own line:
// Hundreds: [digit]
// Tens: [digit]
// Ones: [digit]

fn main() {
    println!("Welcome to Digit extractor app!");

    let input = read_input("Enter three-digit number:");
    let res = decompose(input);
   
    show_res(res);    
}

fn show_res(res: (i32, i32, i32)){
    println!("------");
    println!("Hundreds: {}", res.0);
    println!("Tens: {}", res.1);
    println!("Ones: {}", res.2);
}

fn decompose(mut input: i32) -> (i32, i32, i32){
    let a = input / 100;
    input %= 100;

    let b = input / 10;
    input %= 10;

    let c = input;

    (a, b, c)
}

fn read_input(s: &str) -> i32{
    println!("{s}");

    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("...cannot read your input");
        let trimmed = input.trim();
        
        match trimmed.parse::<i32>() {
            Ok(num) if is_valid(trimmed, 3) => {
                break num
            }
            _ => {
                println!("...invalid input. Try again:");
            }
        }
    }

}

fn is_valid(s: &str, length: i32) -> bool{
    s.len() == length as usize && s.chars().all(|c| c.is_ascii_digit())
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_valid_with_digit_correct(){
        assert!(is_valid("123", 3));
    }

    #[test]
    fn is_valid_with_digit_but_shorter(){
        assert!(!is_valid("12", 3));
    }

    #[test]
    fn is_valid_with_digit_but_longer(){
        assert!(!is_valid("1234", 3));
    }
}