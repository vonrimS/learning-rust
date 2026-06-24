use title_case::*;

fn main() {
    println!("Enter your phrase:");
    let phrase = read_input();

    println!("{}", process_input(&phrase));
}