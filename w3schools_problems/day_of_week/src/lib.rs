// Read a number from input (1 to 7).

// Print the name of the day:

// 1 = Monday
// 2 = Tuesday
// 3 = Wednesday
// 4 = Thursday
// 5 = Friday
// 6 = Saturday
// 7 = Sunday
// If the number is not 1-7, print Invalid.

use std::io;

pub fn read_input() -> u32{
    println!("From 1 to 7, enter your number:");
    
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("...cannot read your input");
        
        if let Ok(day) = input.trim().parse::<u32>() {
            if day > 0 && day <=7 {
                break day;
            }
        }

        println!("...invalid input. Try again:");
    }
}


pub fn day_name(day_num: u32) -> String{
    let day_name: String =  match day_num {
        1 => "Monday".to_string(),
        2 => "Tuesday".to_string(),
        3 => "Wednesday".to_string(),
        4 => "Thursday".to_string(),
        5 => "Friday".to_string(),
        6 => "Saturday".to_string(),
        _ => "Sunday".to_string(),
    };

    day_name
}



#[cfg(test)]

mod tests{
    use super::*;


    #[test]
    fn test_print_day(){
        assert_eq!(day_name(1), "Monday".to_string());
        assert_eq!(day_name(2), "Tuesday".to_string());
        assert_eq!(day_name(7), "Sunday".to_string());

        //impossible scenario due to internal validation in fn read_input() but it works
        assert_eq!(day_name(123), "Sunday".to_string());
    }
}