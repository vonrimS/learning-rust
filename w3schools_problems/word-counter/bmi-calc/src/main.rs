use rust_decimal::Decimal;
use std::io;

// Read a weight in kilograms and a height in meters from input.
// Calculate the BMI using this formula:
// BMI = weight / (height * height)
// Print the result with one decimal place:
// BMI: [result]

fn main() {
    println!("Hello, let's calculate your BMI!");

    let weight = read_user_input("Your weight, in kg:");
    let height = read_user_input("Your height, in meters:");

    println!("BMI: {:.2}", weight / (height * height))
}


fn read_user_input(s: &str) -> Decimal{
    println!("{s}");

    let mut user_input = String::new();
    io::stdin().read_line(&mut user_input).expect("...cannot read your input");
    
    user_input.trim().parse().expect("...cannot pars your input")
}
