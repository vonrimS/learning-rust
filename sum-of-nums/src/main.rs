// The first line of input is a count (how many numbers will follow).
// The next lines each have one number.
// Add all the numbers together and print the sum:
// Sum: [total]

use std::io;

fn sum_nums (mut n: i32) -> i32{
    let mut res: i32 = 0;

    println!("---");
    println!("Enter your every number below:");
    
    while n > 0 {
        let mut num = String::new();
        io::stdin().read_line(&mut num).expect("...cannot read the num");       
        let parsed_num: i32 = num.trim().parse().expect("...cannot parse your input");
        res += parsed_num; 
        n -= 1;
    }

    return res
}

fn main() {
    println!("Welcome to Sum-Of-Numbers app!");

    println!("How many numbers are you going to sum:");
    let mut numbers_input = String::new();
    io::stdin().read_line(&mut numbers_input).expect("...cannot read your input");
    let nums: i32 = numbers_input.trim().parse().expect("...cannot parse your input");

    println!("Sum: [{}]", sum_nums(nums));
}
