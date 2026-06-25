// Read three numbers from input (three lines).
// Write a function that takes three numbers and returns the smallest one.
// Print the result:
// Min: [result]

use std::io;

pub fn read_input(s: &str) -> i32 {
    let mut input = String::new();

    loop {
        input.clear();
        println!("{s}");
        io::stdin()
            .read_line(&mut input)
            .expect("...cannot read your input");

        if let Ok(num) = input.trim().parse() {
            break num;
        }

        println!("...invalid input. Try again:");
    }
}

pub fn grab_nums() -> (i32, i32, i32) {
    let a = read_input("Enter 1st num:");
    let b = read_input("Enter 2nd num:");
    let c = read_input("Enter 3rd num:");
    (a, b, c)
}

pub fn find_min(a: i32, b: i32, c: i32) -> i32 {
    a.min(b).min(c)
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn test_find_min() {
        assert_eq!(find_min(1, 2, 3), 1);
        assert_eq!(find_min(3, 2, 1), 1);
        assert_eq!(find_min(1, 1, 1), 1);
    }
}
