use std::io;

// Read a base and an exponent from input (both are whole numbers, exponent is 0 or more).
// Write a function that calculates base raised to the power of exponent using a loop (do not use a built-in power function).
// Print the result:
// Result: [result]

fn main() {
    let base = read_input("Enter your base:");
    let exp = read_input("Enter your exponent:");

    let res = calculate_power(base, exp);

    println!("Result: {}", res);
}


fn calculate_power(base: u32, mut exp: u32) -> u32 {
    let mut res = 1;

    while exp > 0 {
        res *= base;
        exp -= 1;
    }

    res
}

fn read_input(s: &str) -> u32{
    println!("{s}");

    loop{
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("...cannot read your input");

        if let Ok(num) = input.trim().parse() {
            break num;
        }
        println!("...invalid input. Try again:");
    }
}





#[cfg(test)]

mod tests{
    use super::*;

    #[test]
    fn calculate_power_zero_base(){
        assert_eq!(calculate_power(0, 1), 0);
        assert_eq!(calculate_power(0, 2), 0);
        assert_eq!(calculate_power(0, 10), 0);
    }

    #[test]
    fn calculate_power_zero_exponent() {
        assert_eq!(calculate_power(0, 0), 1);
        assert_eq!(calculate_power(10, 0), 1);
    }

    #[test]
    fn calculate_power_ok() {
        assert_eq!(calculate_power(2, 2), 4);
        assert_eq!(calculate_power(3, 3), 27);
        assert_eq!(calculate_power(5, 2), 25);
    }

    #[test]
    #[should_panic(expected = "overflow")]
    fn test_calculate_power_overflow_panic(){
        calculate_power(10,10);
    }
    
}