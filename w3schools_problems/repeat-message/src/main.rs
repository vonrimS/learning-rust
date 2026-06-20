use text_io::read;

// Read a message and a number N from input.
// Print the message N times, each on its own line.


fn main() {
    println!("Hello, what message do you want to repeat? Type:");
    let message: String = read!();
    
    println!("...and how many times are you goint to repeat it?");
    let mut times: i32 = read!();

    while times > 0{
        println!("{}", message);
        times -= 1;
    }
}
