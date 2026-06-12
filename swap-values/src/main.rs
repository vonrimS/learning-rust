use text_io::read;

// Read two words from input (two lines).
// Print them in swapped order:
// [second]
// [first]

fn main() {
    println!("Enter your first word:");
    let a: String = read!();
    println!("Enter your second word:");
    let b: String = read!();
    println!("-----------");
    println!("{b}");
    println!("{a}");
}
