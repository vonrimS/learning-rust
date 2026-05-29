use std::{cmp::Ordering, io};
use rand::Rng;

fn main() {
    println!("Hello! Let's play a Guess game!");
    println!("Enter your input: ");

    let secret_num = rand::thread_rng().gen_range(0..=10);

    println!("...secret number was generated. Try to guess it in arange between 0 and 10");
    
    loop {

        println!("\nPlease input your guess:");
    
        let mut user_input = String::new();
    
        io::stdin()
            .read_line(&mut user_input)
            .expect("...enter a number");
    
        let user_input: u32 = match user_input.trim().parse(){
            Ok(num) => num,
            Err(_) => continue
        };

        match user_input.cmp(&secret_num){
            Ordering::Less => println!("Less than a number"),
            Ordering::Greater => println!("Greater than a number"),
            Ordering::Equal => {
                println!("Hooray! Equal to a number!");
                break;
            } 
                
        }
    }

}
