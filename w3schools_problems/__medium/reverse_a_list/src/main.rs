// The first line of input is a count (how many numbers will follow).
// The next lines each have one number.
// Print all numbers in reverse order, separated by spaces, on one line.


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
    println!("How many elements are you going to enter:");
    let count = loop {
        let c = read_input();
        if c > 0 {
            break c as usize;
        }

        println!("...your input should be greater than 0. Try again:");
    };

    let mut numbers: Vec<i32> = Vec::with_capacity(count);

    for _ in 0..count {
        numbers.push(read_input())
    }

    numbers
}

fn reverse_dir(v: &[i32]) -> String {
    v.iter()
        .rev()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(" ")
}
fn main() {
    let nums = read_numbers();
    println!("{}", reverse_dir(&nums));
}


#[cfg(test)]

mod tests{
    use super::*;

    #[test]
    fn test_reverse_dir(){
        assert_eq!(reverse_dir(&vec![1,2,3]), "3 2 1");
        assert_eq!(reverse_dir(&vec![3,2,1]), "1 2 3");
        assert_eq!(reverse_dir(&vec![]), "");
    }
}