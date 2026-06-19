// Read three numbers from input (three lines).
// Print the smallest one:
// Smallest: [number]

use std::io;

fn main() {
    println!("Enter three numbers:");
    let a = read_input();
    let b = read_input();
    let c = read_input();
    println!("------");
    println!("Smallest: {}", find_smallest(a, b, c));
}

fn find_smallest(a: i32, b: i32, c: i32) -> i32{
    let mut res = i32::MAX;

    if a < res {
        res = a
    } 
    if b < res {
        res = b
    } 
    if c < res {
        res = c
    }
    res
}


fn read_input() -> i32{
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("...cannot read your input");

        if let Ok(num) = input.trim().parse(){
            break num;
        }

        println!("...invalid input. Try again:");
    }
}



#[cfg(test)]

mod tests{
    use super::*;

    #[test]
    fn test_find_smallest(){
        assert_eq!(find_smallest(0, 0, 0), 0);
        assert_eq!(find_smallest(1, 2, 3), 1);
        assert_eq!(find_smallest(1, 1, 1), 1);
        assert_eq!(find_smallest(1, 10, 100), 1);
    }

}