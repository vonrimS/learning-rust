use text_io::{read};
use rust_decimal::Decimal;

// Read an amount and an exchange rate from input.
// Multiply the amount by the rate to get the result.
// Print the result with two decimal places, like this:
// Result: [result]

fn main() {
    println!("Welcome to our Currency Exchange app!");
    
    println!("...enter amount of money:");
    let ammount: Decimal = read!();
    
    println!("...enter exhange rate:");
    let rate: Decimal = read!();

    println!("Result: {:.2}", ammount * rate);
}
