// Read three side lengths from input (three lines).

// First check if the sides can form a valid triangle. 
// A triangle is valid if the sum of any two sides is greater than the third side.

// If not valid, print Not a triangle.

// If valid, print the type:

// Equilateral if all three sides are equal
// Isosceles if exactly two sides are equal
// Scalene if no sides are equal

use std::io;

pub struct Triangle{
    side_a: u32,
    side_b: u32,
    side_c: u32,
}

pub fn read_input(s: &str) -> u32{
    println!("{s}");

    loop{
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("...cannot read your input");

        if let Ok(num) = input.trim().parse::<u32>(){
            if num > 0 {
                break num;
            }
        }

        println!("...invalid input. Try again:");
    }
}


pub fn create_triangle() -> Triangle {
    Triangle {
        side_a: read_input("Enter value for side A:"),
        side_b: read_input("Enter value for side B:"),
        side_c: read_input("Enter value for side C:"),
    }
}


pub fn is_valid(t: &Triangle) -> bool{
    (t.side_a + t.side_b > t.side_c) && 
    (t.side_a + t.side_c > t.side_b) && 
    (t.side_b + t.side_c > t.side_a)
}

pub fn is_equilateral(t: &Triangle) -> bool {
    t.side_a == t.side_b && t.side_b == t.side_c
}

pub fn is_isosceles(t: &Triangle) -> bool {
    !is_equilateral(t) && (t.side_a == t.side_b || t.side_a == t.side_c || t.side_b == t.side_c)
}

pub fn is_scalene(t: &Triangle) -> bool {
    t.side_a != t.side_b && t.side_b != t.side_c
}


pub fn classify(t: &Triangle) -> String {
    if is_equilateral(&t) {
        "Equilateral".to_string()
    }
    else if is_isosceles(&t) {
        "Isosceles".to_string()
    } 
    else {
        "Scalene".to_string()
    }     
}



#[cfg(test)]

mod tests{
    use super::*;

    #[test]
    fn test_is_valid(){
        assert!(is_valid(&Triangle { side_a: 3, side_b: 4, side_c: 5 }));
        assert!(is_valid(&Triangle { side_a: 2, side_b: 2, side_c: 3 }));
        assert!(is_valid(&Triangle { side_a: 5, side_b: 5, side_c: 5 }));
        assert!(is_valid(&Triangle { side_a: 1, side_b: 1, side_c: 1 }));

        assert!(!is_valid(&Triangle { side_a: 1, side_b: 1, side_c: 2 }));
        assert!(!is_valid(&Triangle { side_a: 1, side_b: 1, side_c: 3 }));
        assert!(!is_valid(&Triangle { side_a: 2, side_b: 2, side_c: 5 }));
    }
}