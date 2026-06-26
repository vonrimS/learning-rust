// The first line of input is a count (how many numbers will follow).
// The next lines each have one number.
// Find the largest number and print it:
// Largest: [number]

use std::io;

fn read_input() -> i32{
    let mut input = String::new();
    loop {
        input.clear();
        if io::stdin().read_line(&mut input).is_ok(){
            if let Ok(num) = input.trim().parse::<i32>(){
                return num;
            }
        }
        println!("...invalid input! Try again:");
    }
}

fn read_numbers() -> Vec<i32>{
    let count = loop {
        let c = read_input();
        if c > 0 {
            break c as usize;
        }
        println!("...count should be greater than 0. Try again:");
    };

    let mut numbers = Vec::with_capacity(count);

    for _ in 0..count{
        numbers.push(read_input());
    }

    numbers
}

fn find_largest(v: &Vec<i32>) -> Option<i32>{
    v.iter().max().copied()
}

fn main(){
    println!("Enter count, then numbers:");
    let numbers = read_numbers();

    if let Some(largest) = find_largest(&numbers) {
        println!("Largest: {}", largest);
    } else {
        println!("...no numbers were entered");
    }
}


#[cfg(test)]

mod tests{
    use super::*;

    #[test]
    fn test_find_largest(){
        assert_eq!(find_largest(&vec![1, 2, 3]), Some(3));
        assert_eq!(find_largest(&vec![3, 2, 1]), Some(3));
        assert_eq!(find_largest(&vec![1, 3, 2]), Some(3));

        assert_eq!(find_largest(&vec![0, 0, 0]), Some(0));
        assert_eq!(find_largest(&vec![-1, -10, -2]), Some(-1));
        
        assert_eq!(find_largest(&vec![] as &Vec<i32>), None);
    }
}