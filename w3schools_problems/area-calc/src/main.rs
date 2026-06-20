// Read a shape and its measurements from input.
// The shape is one of these three words:
// rectangle - next two lines are width and height
// triangle - next two lines are base and height
// circle - next line is the radius
// Print the area with two decimal places:
//  Area: [result]

use core::f64;
use std::io;

fn count_ractangle_area() -> f64 {

    println!("side a:");
    let mut side_a = String::new();
    io::stdin().read_line(&mut side_a).expect("...some problems");
    let side_a: f64 = side_a.trim().parse().expect("...some problems");
    
    println!("side b:");
    let mut side_b = String::new();
    io::stdin().read_line(&mut side_b).expect("...some problems");
    let side_b: f64 = side_b.trim().parse().expect("...some problems");

    side_a * side_b
}

fn count_triangle_area() -> f64 {
    println!("base:");
    let mut base = String::new();
    io::stdin().read_line(&mut base).expect("...cannot read your input");
    let base: f64 = base.trim().parse().expect("...cannot parse it");
    
    println!("height:");
    let mut height = String::new();
    io::stdin().read_line(&mut height).expect("...cannot read your input");
    let height: f64 = height.trim().parse().expect("...cannot parse it");

    0.5 * base * height
}

fn count_circle_area() -> f64 {
    println!("radius:");
    let mut radius = String::new();
    io::stdin().read_line(&mut radius).expect("...cannot read your input");
    let radius: f64 = radius.trim().parse().expect("...cannot parse it");

    f64::consts::PI * radius * radius
}

fn shape_area (input: &str) -> f64 {
    match input {
        "a" => count_ractangle_area(),
        "b" => count_triangle_area(),
        "c" => count_circle_area(),
        _ => 0.0
    }
}


fn main() {
    println!("Welcome to Area calculator!");
    println!("Choose your shape which area your are going to count:");
    println!("a - rectangle | b - triangle | c - circle");
    
    let mut user_input = String::new();
    io::stdin().read_line(&mut user_input).expect("...some problem");
    let user_input = user_input.trim();
    

    println!("Area: [{}]", shape_area(user_input));

}
