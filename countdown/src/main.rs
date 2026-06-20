// Read a number from input.
// Count down from that number to 1, then print Go!
// Each number should be on its own line.


use std::io;

fn main() {
    println!("Enter a number to countdown:");
    let mut n = read_input();

    println!("------");
    while n > 0 {
        println!("{n}");
        n -= 1;
    }

    println!("Go!");
}


fn read_input() -> i32{
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("...cannot read your input");

        if let Ok(num) = input.trim().parse(){
            if num > 0 {
                break num;
            }
        }
        println!("...invalid input. Try again:");
    }
}
