// Read a number from input.
// Print one of these words:
// Positive if the number is greater than 0
// Negative if the number is less than 0
// Zero if the number is 0

use std::io;


fn main() {
    println!("Enter some number:");
    let input = read_input();

    println!("{}", evaluate_val(input));
}

fn evaluate_val(val: i32) -> String {
    match val {
        0 => String::from("Zero"),
        1.. => String::from("Positive"),
        ..=-1 => String::from("Negative")
    }
}


fn read_input() -> i32{
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("...cannot read your input");

        if let Ok(num) = input.trim().parse() {
            break num;
        }

        println!("...invalid input. Try again:");
    }
}



#[cfg(test)]

mod tests{
    use super::*;

    #[test]
    fn test_evaluate_val(){
        assert_eq!(evaluate_val(0), String::from("Zero"));
        assert_eq!(evaluate_val(1), String::from("Positive"));
        assert_eq!(evaluate_val(-1), String::from("Negative"));
    }
}