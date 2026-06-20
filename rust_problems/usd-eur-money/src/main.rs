// Create two distinct types, USD and EUR, as tuple structs wrapping an f64.
// Write a function double_usd(amount: USD) -> USD. 
// Ensure that if a developer accidentally passes a EUR instance into this function, 
// the code fails to compile, preventing currency mixing at compile time.

use usd_eur_money::{double_usd, read_input, USD};

fn main() {
    println!("Welcome to Currency Exchange App!");
    let usd = USD(read_input("Enter your USD amount:"));
    // let eur = EUR(read_input("Enter your EUR amount:"));

    println!("{:?}", double_usd(usd));
    // println!("{:?}", double_usd(eur));
}