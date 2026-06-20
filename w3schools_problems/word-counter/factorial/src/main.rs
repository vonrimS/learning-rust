
// Read a number from input.
// Create a function that calculates the factorial of that number.
// The factorial of N means: multiply all numbers from 1 to N together.
// Example: factorial of 5 = 1 × 2 × 3 × 4 × 5 = 120
// The factorial of 0 is 1.
// Print the result like this:
// [n]! = [result]

use std::io;

fn get_factorial(mut n: i64) -> i64 {
    let mut res= 1;
    
    if n > 1 {
        while n > 0{
            res *= n;
            n -= 1;
        }
    }

    return res
}


fn main() {
    println!("For calculating factorial, enter a number:");

    let mut target = String::new();
    io::stdin().read_line(&mut target).expect("...cannot read your input");
    let n = target.trim().parse().expect("...cannot parse your input");

    println!("[{}]! = [{}]", n, get_factorial(n));
}
