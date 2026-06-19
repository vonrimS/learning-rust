// Your program reads two lines:
// The bill amount (a number like 50.00)
// The tip percentage (a whole number like 18)
// Use a function to calculate the tip and total, then print these 3 lines (all with two decimal places):
// Bill: $[bill]
// Tip: $[tip]
// Total: $[total]

use std::io;

fn main() {
    let bill = read_input("Enter bill amount:");
    let tip = read_input("Enter tip percentage:");

    let tip_money = calc_tip_money(bill, tip);
    let total = calc_total(bill, tip_money);

    println!("------");
    println!("Bill: ${:.2}", bill);
    println!("Tip: ${:.2}", tip_money);
    println!("Total: ${:.2}", total);
}

fn calc_tip_money(bill: f64, tip: f64) -> f64 {
    bill * tip / 100 as f64
}

fn calc_total(bill: f64, tip_money: f64) -> f64 {
    bill + tip_money
}

fn read_input(s: &str) -> f64 {
    println!("{}", s);

    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("...cannot read your input");

        if let Ok(num) = input.trim().parse::<f64>(){
            if num > 0 as f64 {
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
    fn test_calc_tip_money(){
        assert_eq!(calc_tip_money(100 as f64, 10 as f64), 10 as f64);
    }
}
