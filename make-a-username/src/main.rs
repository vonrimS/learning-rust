use std::io;

// Read a first name and a last name from input.
// Create a username by joining the two names together in lowercase (no space between them).
// Print these two lines:
//  Username: [username] (in lowercase)
//  Initials: [first letter of first name][first letter of last name] (in uppercase)

fn main(){
    
    // first name
    println!("Enter your [first name]:");
    let mut first_name = String::new();
    io::stdin().read_line(&mut first_name).expect("...cannot read your input");
    
    // last name
    println!("Enter your [last name]:");
    let mut last_name = String::new();
    io::stdin().read_line(&mut last_name).expect("...cannot read your input");

    let username = first_name.trim().to_owned() + last_name.trim();

    let f_char = first_name.chars().next().unwrap().to_string();
    let l_char = &last_name.chars().next().unwrap().to_string();

    let initials = f_char.to_owned() + l_char;

    println!("Username: {}", username.to_lowercase());
    println!("Initials: {}", initials.to_uppercase());
}