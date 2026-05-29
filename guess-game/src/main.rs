use std::io;
use rand::Rng;

fn main() {
    println!("Hello! Let's play a Guess game!");
    println!("Enter your input: ");

    let secret_num = rand::thread_rng().gen_range(0..=10);

    println!("...secret number was generated. Try to guess it in arange between 0 and 10");

    let mut user_input = String::new();

    io::stdin().read_line(&mut user_input).expect("...enter a number!");

    println!("Your input was: {user_input}");

    println!("...and the secrete number was: {secret_num}");
}
