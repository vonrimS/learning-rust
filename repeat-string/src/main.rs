// Read a word and a number N from input.
// Print the word repeated N times, all joined together on one line (no spaces).

use std::io;
use std::str::FromStr;

fn main() {
    let word: String = read_input("Enter a word:");
    let n: usize = read_input("Enter a number:");
    let res = word.repeat(n);
    println!("------");
    println!("Result: {}", res);
}


fn read_input<T> (s: &str) -> T
where
    T: FromStr
{
    println!("{s}");
    
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("...cannot read your input");
        
        if let Ok(value) = input.trim().parse::<T>() {
            break value;
        }
        println!("...invalid value. Try again:");
    }
}

