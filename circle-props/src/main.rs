use std::{f64::consts::PI, io};



// Read a radius from input.
// Calculate the circumference and area of the circle:
//      Area = pi × radius × radius
//      Circumference = 2 × pi × radius
// Print the results with two decimal places:
//      Area: [result]
//      Circumference: [result]


fn main() {
    println!("What is your radius?");

    let radius = input_radius();

    let area = circle_area(radius);
    let circumference = cirst_circumference(radius);

    show_metrics(area, circumference);
}


fn show_metrics(area: f64, circumference: f64){
    println!("Area: {:.2}", area);
    println!("Circumference: {:.2}", circumference);
}

fn input_radius() -> f64 {
    println!("Enter radius:");
    let mut radius = String::new();

    io::stdin().read_line(&mut radius).expect("...cannot read your input");
    radius.trim().parse().expect("...cannot parse your input")
}


fn circle_area(r: f64) -> f64{
    PI * r * r
}

fn cirst_circumference(r: f64) -> f64{
    2 as f64 * PI * r
}





