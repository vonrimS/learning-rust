use std::io;

// Read two numbers from input.
// Print the absolute difference (always a positive number):
// Difference: [result]

fn main() {
    println!("Welcome to the Absolute Difference App!");

    let a = read_input("Enter number A:");
    let b = read_input("Enter number B:");

    let diff = a - b;

    println!("Difference: {}", absolute_value(diff));
}


fn read_input(s: &str) -> i32{
    println!("{s}");

    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("...cannot read your input");

        if let Ok(num) = input.trim().parse() {
            break num;
        }

        println!("...invalid input. Try agin: ");
    }
}

fn absolute_value(a: i32) -> u32 {
    let res;
    if a < 0 {
        res = a * -1;
    } else {
        res = a;
    }
    res as u32
}


#[cfg(test)]

mod tests{
    use super::*;

    #[test]
    fn test_absolute_value_zero(){
        assert_eq!(absolute_value(0), 0);
    }

    #[test]
    fn test_absolute_value_negative(){
        assert_eq!(absolute_value(-1), 1);
        assert_eq!(absolute_value(-100), 100);
        assert_eq!(absolute_value(-1000), 1000);
    }

    #[test]
    fn test_absolute_value_positive(){
        assert_eq!(absolute_value(1), 1);
        assert_eq!(absolute_value(100), 100);
        assert_eq!(absolute_value(1000), 1000);
    }
}