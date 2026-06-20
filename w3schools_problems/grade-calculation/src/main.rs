// Read a score from input (0 to 100).
// Print the letter grade:
//  90 or more: A
//  80 to 89: B
//  70 to 79: C
//  60 to 69: D
//  Below 60: F

use std::io;

fn main() {
    println!("Welcome to grade calc!");
    println!("What is your score?");

    let mut user_score = String::new();
    io::stdin().read_line(&mut user_score).expect("...cannot read your input");

    let score: u32 = user_score.trim().parse().expect("...cannot parse your input");

    let grade = if score >= 90 {
        "A"
    } else if score >= 80 && score < 90 {
        "B"
    } else if score >= 70 && score < 80 {
        "C"
    } else if score >= 60 && score < 70 {
        "D"
    } else {
        "F"
    };

    println!("With a score of {}, you are getting an \'{}\' grade", score, grade);
}
