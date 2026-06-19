use std::io;

// Read a number N from input.
// Calculate the sum of all numbers from 1 to N.
// Print the result:
// Sum: [result]

fn main() {
    println!("We are going to sum all numbers in range of 1 to N.");
    println!("Enter a number N:");
    let n = read_input();
    let sum = sum_all_nums(n);

    println!("Sum: {}", sum);
}

fn sum_all_nums(n: u32) -> u32 {
    let mut sum= 0;
    for number in 1..=n {
        sum += number;
    }

    sum
}

fn read_input() -> u32{
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



#[cfg(test)]

mod tests{
    use super::*;

    #[test]
    fn test_sum_all_nums(){
        assert_eq!(sum_all_nums(1), 1);
        assert_eq!(sum_all_nums(2), 3);
        assert_eq!(sum_all_nums(3), 6);
        assert_eq!(sum_all_nums(4), 10);
    }
}