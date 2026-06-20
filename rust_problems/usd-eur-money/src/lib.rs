use std::io;


#[derive(Debug, PartialEq)]
pub struct USD(pub f64);

#[derive(Debug, PartialEq)]
pub struct EUR(pub f64);


pub fn read_input(s: &str) -> f64{
    println!("{s}");

    loop{
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("...cannot read your input");

        if let Ok(num) = input.trim().parse::<f64>() {
            if num > 0_f64 {
                break num;
            }
        }

        println!("...invalid input. Try again:");
    }
}


pub fn double_usd(amount: USD) -> USD {
    USD(amount.0 * 2 as f64)
}



#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn test_double_usd_with_usd(){
        assert_eq!(double_usd(USD(1_f64)), USD(2_f64));
        assert_eq!(double_usd(USD(10_f64)), USD(20_f64));
        assert_eq!(double_usd(USD(20_f64)), USD(40_f64));
    }

}