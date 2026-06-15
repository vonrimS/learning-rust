use std::io;
use rust_decimal::{self, Decimal};

// Read a price and a discount percentage from input.
// Calculate the discount amount and the final price.
// Print these two lines (two decimal places):
// Discount: [amount]
// Final price: [price]

fn main() {
    println!("Hello, welcome to our store!");

    let price = read_price_input();
    let discount = read_discount_input();

    let discount_break_down = price_discount(price, discount);

    print_out(&discount_break_down);

}

fn print_out(tup: &(Decimal, Decimal)){
    println!("---------");
    println!("Discount: {:.2}", tup.0);
    println!("Final price: {:.2}", tup.1);
}


fn read_price_input() -> Decimal{
    println!("Enter full price:");
    
    loop {
        let mut price = String::new();
        io::stdin().read_line(&mut price).expect("...cannot read your input");

        match price.trim().parse() {
            Ok (num) => {
                if num > Decimal::from(0) {
                    break num;
                } else {
                    println!("...price should be greater than null. Try again:")
                }
            }
            Err(_) => {
                println!("...invalid number. Try agan:");
            }
        }
    }
}


fn read_discount_input() -> Decimal {
    println!("Enter discount, in %:");
    
    loop{    
        let mut discount = String::new();
        io::stdin().read_line(&mut discount).expect("...cannot read your input");

        let min_discount = Decimal::ZERO;
        let max_discount = Decimal::from(100);

        match discount.trim().parse() {
            Ok(num) => {
                if num >= min_discount && num <= max_discount {
                    break num;
                } else {
                    println!("...discount must be between 0 and 100. Try again:");
                }
            }              
            Err(_) => {
                println!("...invalid number. Try again:");
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