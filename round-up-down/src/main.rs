use std::io;

// Read a decimal number from input.
// Print it rounded to the nearest whole number:
// Rounded: [result]

fn main() {
    println!("Welcome to Round Decimal App!");

    println!("Enter your decimal below:");
    let input = read_decimal_input();

    println!("{}", input.round());
}


fn read_decimal_input() -> f64 {
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("...cannot read your input");    

        if let Ok(val) = input.trim().parse::<f64>() {    
            break val;                
        }

        println!("...invalid input. Try again:");
    }
}

