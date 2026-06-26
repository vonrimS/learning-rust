// Read a positive number N from input.
// Write a recursive function that calculates the sum of all numbers from 1 to N.
// The function must call itself (no loops allowed in the function).
// Print the result:
// Sum: [result]

use std::io;

pub fn read_input() -> u32 {
    println!("Enter how deep is yor recursive counting:");

    let mut input = String::new();
    loop {
        input.clear();
        io::stdin()
            .read_line(&mut input)
            .expect("...cannot read your input");

        if let Ok(num) = input.trim().parse::<u32>() {
            if num > 0 {
                break num;
            }
        }

        println!("...invalid input. Try again:");
    }
}

pub fn recursive_count(n: u32) -> u32 {
    if n == 1 {
        return 1;
    }
    n + recursive_count(n - 1)
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn test_recursive_count() {
        assert_eq!(recursive_count(1), 1);
        assert_eq!(recursive_count(2), 3);
        assert_eq!(recursive_count(3), 6);
    }
}
