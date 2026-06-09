// Read a name and an age from input.
// If the person is 18 or older, print:
// [name] can vote
// If the person is younger than 18, print:
// [name] cannot vote

use std::io;

fn main() {
    println!("What is your name?");
    let mut name = String::new();
    io::stdin().read_line(&mut name).expect("...cannot read your input");

    println!("How old are you?");
    let mut age = String::new();
    io::stdin().read_line(&mut age).expect("...cannot read your input");

    let age: i32 = age.trim().parse().expect("...it's not a number");

    if age < 18 {
        println!("...sorry, you cannot vote.");
    } else {
        println!("Everything is ok! You can vote!")
    }
}
