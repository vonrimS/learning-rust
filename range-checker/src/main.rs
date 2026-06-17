use std::io;

// Read three numbers from input: a number, a low value, and a high value.
// Check if the number is between low and high (including both ends).
// Print In range or Out of range.

fn main() {
    let num_value = read_input("Enter your target num:");
    let range_min_value = read_input("Enter your range low value:");
    let range_max_value = read_input("Enter your range high value:");

    if within_range(num_value, range_min_value, range_max_value) {
        println!("In range");
    } else {
        println!("Out of range");
    }

}

fn within_range(num: i32, range_min: i32, range_max: i32) -> bool{
    num >= range_min && num <= range_max
}


fn read_input(s: &str) -> i32{
    println!("{s}");
    
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("...cannot read your input");

        if let Ok(num) = input.trim().parse(){
            break num;
        }
        println!("...invalid input. Try again:");
    }
}


#[cfg(test)]

mod tests{
    use super::*;

    #[test]
    fn within_range_ok(){
        assert!(within_range(3, 2, 4))
    }

    #[test]
    fn within_range_edge(){
        assert!(within_range(2, 2, 4));
        assert!(within_range(4, 2, 4));
    }

    #[test]
    fn within_range_nok(){
        assert!(!within_range(1, 2, 4));
        assert!(!within_range(5, 2, 4));
    }
}