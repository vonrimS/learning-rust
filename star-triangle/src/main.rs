use std::io;

// Read a number N from input.
// Print N rows. Row 1 has 1 star, row 2 has 2 stars, and so on.
// Each row contains only * characters (no spaces).

fn main() {
    println!("We are goint to draw a Star Triangle.");

    println!("How many lines?");
    let n = read_input();
    let vctr = fill_piramid(n);

    for element in vctr {
        println!("{}", element);
    }
}


fn fill_piramid(n: i32) -> Vec<String>{
    let mut res: Vec<String> = Vec::new();
    let mut brick = String::from("*");

    for _ in 1..=n {
        res.push(brick.clone());
        brick.push_str("*");
    }

    res
}

fn read_input() -> i32{
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("...cannot read your input");

        if let Ok(num) = input.trim().parse::<i32>() {
            if num > 0 {
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
    fn test_fill_piramid(){
        assert_eq!(fill_piramid(1), vec!["*"]);
        assert_eq!(fill_piramid(2), vec!["*", "**"]);
        assert_eq!(fill_piramid(3), vec!["*", "**", "***"]);
    }
}