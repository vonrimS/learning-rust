use std::io;

fn main() {
    println!("Hello! Let's play a Guess game!");
    println!("Enter your input: ");

    let mut user_input = String::new();

    io::stdin().read_line(&mut user_input).expect("...enter a number!");

    println!("Your input was: {user_input}");
}
