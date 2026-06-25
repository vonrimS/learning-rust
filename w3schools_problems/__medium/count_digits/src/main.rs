use count_digits::*;

fn main() {
    let digits = read_input();
    let count = count_digits(&digits);
    println!("Count: {}", count)
}
