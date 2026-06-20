use text_io::read;
use rust_decimal::Decimal;


// Your program reads three lines:

// Item name
// Price (a number like 1.50)
// Quantity (a whole number like 3)
// Print exactly these 4 lines:

// Item: [item]
// Price: $[price] (always two decimal places, like 1.50)
// Quantity: [quantity]
// Total: $[total] (price times quantity, two decimal places)

fn main() {
    println!("Welcome to our shop!");
    println!("Enter your [item name], its [price] (USD) and [quantity]:");
    
    let item_name: String = read!();

    // cannot use f32, not suitable for counting money
    // let price: f32 = read!(); 
    // let quantity: f32 = read!();
    
    let price: Decimal = read!();
    let quantity: Decimal = read!();

    // Receipt output
    println!("Item: {}", item_name);
    println!("Price: ${:.2}", price);
    println!("Quantity: {}", quantity);
    println!("Total: ${:.2}", price * quantity);
}
