use text_io::read;

// Read a name, an age, and a city from input (three lines).
// Print these three lines:
// Name: [name]
// Age: [age]
// City: [city]


fn main() {
    println!("Hello! Please share your personal info.");

    println!("Your name:");
    let name: String = read!();

    println!("Your age:");
    let age: i32 = read!();

    println!("Your city of residence:");
    let city: String = read!();

    println!("---------");

    println!("Name: {}" , name);
    println!("Age: {}" , age);
    println!("City: {}" , city);


}
