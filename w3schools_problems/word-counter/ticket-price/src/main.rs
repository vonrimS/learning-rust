use std::io;
use rust_decimal::Decimal;

// A movie theater has different prices based on age:
// Child (under 12 years old): $5
// Adult (12 to 64 years old): $15
// Senior (65 years old or more): $8
// Read an age from input and print these two lines:
// [type]
// $[price]


#[derive(PartialEq, Debug)]
enum Grade {
    Child,
    Adult,
    Senior
}

fn main() {
    println!("Welcome to Box office App!");
    let age = read_input("Enter your age: ");
    let age_grade = age_graded(age);
    let price = price_graded(&age_grade);

    println!("------");
    println!("Age: {}", age_graded_display(&age_grade));
    println!("Price: ${:.2}", price);
}

fn age_graded_display(age_grade: &Grade) -> String {
    let grade: &str = match age_grade {
        Grade::Child => "Child",
        Grade::Adult => "Adult",
        _ => "Senior",
    };

    grade.to_string()
}


fn age_graded(age: i32) -> Grade {
    match age {
        ..12 => Grade::Child,
        12..65 => Grade::Adult,
        _ => Grade::Senior        
    }
}

fn price_graded(grade: &Grade) -> Decimal {
    let price = match grade {
        Grade::Child => 5,
        Grade::Adult => 15,
        _ => 8
    };

    Decimal::from(price)
}

fn read_input(s: &str) -> i32{
    println!("{s}");

    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("...cannot read your input");

        if let Ok(num) = input.trim().parse() {
            break num;
        }
        println!("...invalid input. Try again:");
    }
}


#[cfg(test)]

mod tests{
    use super::*;


    #[test]
    fn age_graded_ok(){
        assert_eq!(age_graded(5), Grade::Child);
        assert_eq!(age_graded(11), Grade::Child);

        assert_eq!(age_graded(12), Grade::Adult);
        assert_eq!(age_graded(64), Grade::Adult);
        
        assert_eq!(age_graded(65), Grade::Senior);
        assert_eq!(age_graded(99), Grade::Senior);
    }

 
    #[test]
    fn price_graded_ok(){
        assert_eq!(price_graded(&Grade::Child), Decimal::from(5));
        assert_eq!(price_graded(&Grade::Adult), Decimal::from(15));
        assert_eq!(price_graded(&Grade::Senior), Decimal::from(8));
    }
}