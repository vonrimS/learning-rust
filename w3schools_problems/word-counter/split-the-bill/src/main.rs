use std::io;
use rust_decimal::Decimal;

// Read a total amount and the number of people from input.
// Calculate how much each person pays.
// Print the result with two decimal places:
// Guests: [guest_qnt]
// Everyone pays: [amount]

use std::{fmt::Display, str::FromStr};

fn main() {
    println!("Hello, let's split the bill!");

    let guests: i32 = read_user_input("How many guests:", (1 as i32, 100 as i32));
    let bill: Decimal = read_user_input("How much in the bill:", (Decimal::from(1), Decimal::MAX));

    println!("------");
    println!("Guests: {}", guests);
    println!("Everyone pays: {:.2}", split_the_bill(guests, bill).unwrap_or(Decimal::ZERO));
}

fn split_the_bill(guests: i32, bill: Decimal) -> Option<Decimal>{
    if guests == 0 {
        None
    } else {
        Some(bill / Decimal::from(guests))
    }
}


fn read_user_input<T>(s: &str, range: (T, T)) -> T
where
    T: FromStr + PartialOrd + Display
{
    println!("{s}");
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("...cannot read your input");

        match input.trim().parse() {
            Ok(num) => {
                if num >= range.0 && num <= range.1 {
                    break num;
                } else {
                    println!("...out of the range.Try again:");
                }
            },
            Err(_) => {
                println!("...invalid input. Try again:");
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normal_split_the_bill(){
        let guests = 10;
        let bill = Decimal::from(100);

        let split = split_the_bill(guests, bill);

        assert_eq!(split, Some(Decimal::from(10)));
    }


}
