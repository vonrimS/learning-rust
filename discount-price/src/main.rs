use std::io;
use rust_decimal::{self, Decimal};

// Read a price and a discount percentage from input.
// Calculate the discount amount and the final price.
// Print these two lines (two decimal places):
// Discount: [amount]
// Final price: [price]

fn main() {
    println!("Hello, welcome to our store!");

    let price_valid_range: (Decimal, Decimal) = (Decimal::from(0), Decimal::MAX);
    let discount_valid_range: (Decimal, Decimal) = (Decimal::from(0), Decimal::from(100));

    let price = read_user_input("Enter original price:", price_valid_range);
    let discount = read_user_input("Enter discount percentage:", discount_valid_range);

    let discount_break_down = price_discount(price, discount);

    print_out(&discount_break_down);
}

fn print_out(tup: &(Decimal, Decimal)){
    println!("---------");
    println!("Discount: {:.2}", tup.0);
    println!("Final price: {:.2}", tup.1);
}


fn read_user_input(s: &str, range:(Decimal, Decimal)) -> Decimal{
    println!("{s}");
    
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("...cannot read your input");

        match input.trim().parse() {
            Ok (num) => {
                if num >= range.0 && num <= range.1 {
                    break num;
                } else {
                    println!("...invalid value. Try again:")
                }
            }
            Err(_) => {
                println!("...invalid number. Try agan:");
            }
        }
    }
}


fn price_discount(price: Decimal, discount: Decimal) -> (Decimal, Decimal){
    let minus_price = price * discount / Decimal::from(100);
    let final_price =price - minus_price;
    let tup = (minus_price, final_price);
    tup
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normal_discount(){
        let price = Decimal::from(100);
        let discount = Decimal::from(10);

        let (minus_price, final_price) = price_discount(price, discount);

        assert_eq!(minus_price, Decimal::from(10));
        assert_eq!(final_price, Decimal::from(90));
    }

    #[test]
    fn test_zero_discount(){
        let price = Decimal::from(100);
        let discount = Decimal::from(0);

        let (minus_price, final_price) = price_discount(price, discount);

        assert_eq!(minus_price, Decimal::from(0));
        assert_eq!(final_price, Decimal::from(100));
    }

    #[test]
    fn test_hundred_discount(){
        let price = Decimal::from(100);
        let discount = Decimal::from(100);

        let (minus_price, final_price) = price_discount(price, discount);

        assert_eq!(minus_price, Decimal::from(100));
        assert_eq!(final_price, Decimal::from(0));
    }
}