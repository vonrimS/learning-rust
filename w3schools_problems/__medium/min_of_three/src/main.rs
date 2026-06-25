use min_of_three::*;

fn main() {
    let (a, b, c) = grab_nums();
    println!("Min: {}", find_min(a, b, c));
}
