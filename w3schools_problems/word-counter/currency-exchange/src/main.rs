use std::io;
use rust_decimal::Decimal;

// Read an amount and an exchange rate from input.
// Multiply the amount by the rate to get the result.
// Print the result with two decimal places, like this:
// Result: [result]

fn main() {
    println!("Welcome to our Currency Exchange app!");    

    let amount: Decimal = read_decimal_from_user("How much:");
    let rate: Decimal = read_decimal_from_user("What's the rate:");

    println!("Result: {:.2}", amount * rate);
}

fn read_decimal_from_user(s: &str) -> Decimal{
    println!("{s}");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("...cannot read your input");
    input.trim().parse().expect("...cannot parse your input")    
}
