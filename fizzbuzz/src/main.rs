// Read a number N from input.
// Print every number from 1 to N, but:
// If the number is divisible by both 3 and 5, print FizzBuzz instead
// If the number is divisible by 3, print Fizz instead
// If the number is divisible by 5, print Buzz instead
// Print each result on its own line.

use std::io;

fn main() {
    println!("Hello, welcome to FizzBuzz kingdom!");
    println!("Enter your target (positive) number:");
    let n = read_input();

    println!("------");

    for element in 1..n {
        println!("{}", inspect_number(element));
    }
}

fn inspect_number(n: i32) -> String {
    match n {
        _ if n%3 == 0 && n%5 == 0 => "FizzBuzz".to_string(),
        _ if n%3 == 0 => "Fizz".to_string(),
        _ if n%5 == 0 => "Buzz".to_string(),
        _ => n.to_string()
    }
}

fn read_input() -> i32 {
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("...cannot read your input");
        if let Ok(num) = input.trim().parse(){
            if num > 0 {
                break num;
            }
        }

        println!("...invalid input. Try again:");
    }
}



