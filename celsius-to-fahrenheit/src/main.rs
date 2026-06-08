use std::io;

// Convert a temperature from Celsius to Fahrenheit.
// Read a temperature in Celsius from input.
// Use this formula:
// fahrenheit = celsius * 9/5 + 32
// Print the result like this:
// [celsius] Celsius = [fahrenheit] Fahrenheit

fn main() {
    
    println!("Welcome to Celsius-to-Fahrenheit calc.");
    println!("Enter you temperature in Celsius. Or type 'q' to quit.");

    loop {

        let mut temp_input = String::new();

        io::stdin()
            .read_line(&mut temp_input)
            .expect("Cannot read your input");

        let trimmed_input = temp_input.trim();

        if trimmed_input == "q".to_lowercase(){
            println!("...quit");
            break;
        }

        let temp_cel: f64 = match trimmed_input.parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        let temp_fah = temp_cel * 9.0/5.0 + 32.0;


        println!("{}°C = {}°F", temp_cel, temp_fah);

    }


}
