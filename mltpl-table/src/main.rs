// Read a number from input.
// Print its multiplication table from 1 to 10.
// Each line should look like this:
// [number] x [i] = [result]

use core::num;
use std::io;

fn main() {
    println!("------------");
    println!("Enter your number: ");

    let mut user_input = String::new();
    io::stdin().read_line(&mut user_input).expect("...cannot read your input");
    let number: i32 = user_input.trim().parse().expect("...cannnot parse your input");

    println!("------------");

    let mut i = 1;

    while i <= 10 {        
        println!("{} x {} = {}", number, i, number * i);
        i = i + 1;
    }

}
