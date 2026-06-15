use std::io;

// Read two numbers from input.
// Print the result of adding, subtracting, multiplying, and dividing them:
// [a] + [b] = [result]
// [a] - [b] = [result]
// [a] * [b] = [result]
// [a] / [b] = [result]

fn main() {
    println!("Hello, welcome to our Simple calc app!");

    let a: i32 = read_user_input("Enter number A:");
    let b: i32 = read_user_input("Enter number B:");

    let sum = add(a, b);
    let diff = substract(a, b);
    let mult = multiply(a, b);
    let quot = divide(a, b);

    println!("{} + {} = {}", a, b, sum);
    println!("{} - {} = {}", a, b, diff);
    println!("{} * {} = {}", a, b, mult);
    println!("{} / {} = {:.2}", a, b, quot);
}

fn read_user_input(s: &str) -> i32{
    println!("{s}");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect(".cannot read your input");
    input.trim().parse().expect("...cannot parse your input")
}

fn add(a: i32, b: i32) -> i32{
    a + b
}

fn substract(a: i32, b: i32) -> i32{
    a - b
}

fn multiply(a: i32, b: i32) -> i32{
    a * b
}

fn divide(a: i32, b: i32) -> f32{
    a as f32 / b as f32
}

