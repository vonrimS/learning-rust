// The first line of input is a count (how many numbers will follow).
// The next lines each have one number.
// Write a function that takes an array of numbers and returns a new array where each number is doubled.
// Print the doubled numbers, separated by spaces, on one line.

use std::io;


fn read_single_int() -> i32{
    let mut input = String::new();
    loop {
        input.clear();
        if io::stdin().read_line(&mut input).is_ok() {
            if let Ok(num) = input.trim().parse::<i32>(){
                return num;
            }
        }
        println!("...invalid input! Try again:");
    }
}

fn read_numbers() -> Vec<i32>{
    let count = loop {
        let c = read_single_int();
        if c > 0{
            break c as usize;
        }
        println!("Count must be greater than 0. Try again:")
    };

    let mut numbers = Vec::with_capacity(count);

    for _ in 0..count {
        numbers.push(read_single_int());
    }

    numbers
}

fn double_numbers(arr: &[i32]) -> Vec<i32>{
    arr.iter().map(|&x| x * 2).collect()
}

fn format_numbers(arr: &[i32]) -> String {
    let strings: Vec<String> = arr.iter().map(|x| x.to_string()).collect();
    strings.join(" ")
}

fn main() {
    let numbers = read_numbers();
    println!("{}", format_numbers(&double_numbers(&numbers)))
}



#[cfg(test)]

mod test{
    use super::*;

    #[test]
    fn test_double_numbers(){
        let input = vec![1, 2, 3];
        let output = vec![2, 4, 6];
        assert_eq!(double_numbers(&input), output);
    }

    #[test]
    fn test_double_numbers_empty(){
        let input = vec![];
        let output = vec![];
        assert_eq!(double_numbers(&input), output);
    }

    #[test]
    fn test_format_numbers(){
        let input = vec![2, 4, 6];
        let output = "2 4 6".to_string();
        assert_eq!(format_numbers(&input), output);
    }

}

