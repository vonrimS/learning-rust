use std::io;

// Read a number from input.
// Print Even if the number is divisible by 2.
// Print Odd if it is not.

fn main() {
    println!("Welcome to even-odd program!");

    loop {
        println!("Enter your input:");

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("...cannot read your input");

        let trimmed = input.trim();

        if trimmed.to_lowercase() == "q"{
            println!("...quit");
            break;
        }

        let user_number: i32 = match trimmed.parse(){
            Ok(num) => num,
            Err(_) => continue,
        };

        if user_number %2 == 0{
            println!("...is an even number");
        } else {
            println!("...is an odd number");
        };

    }

}
