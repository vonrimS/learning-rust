use std::io;

// Read a word from input.
// Print it in uppercase, then print its length:
// [word in uppercase]
// Length: [number]

fn main() {
    println!("Hello, welcome to our Shout-it-out App.");
    let phrase = read_input("Enter your phrase:");

    println!("------");
    print_uppercase(&phrase);
    print_len(&phrase);
}

fn read_input(s: &str) -> String{
    println!("{s}");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("...cannot read your input");
    input.trim().to_string()
}


fn print_uppercase(s: &str){
    println!("{}", s.to_uppercase());
}

fn print_len(s: &str){
    println!("{}", s.len());
}