// The first line of input is a count (how many numbers will follow).
// The next lines each have one number.
// Find the second largest number and print:
// Second largest: [number]
// If two numbers share the highest value, the second largest is still that same value.

use std::io;

fn read_input() -> i32{
    let mut input = String::new();
    loop {
        input.clear();

        if io::stdin().read_line(&mut input).is_ok() 
            && let Ok(num) = input.trim().parse::<i32>(){
                return num;
            }
        
        println!("...invalid input! Try again:");
    }
}

fn read_numbers() -> Vec<i32>{
    println!("How many nums in your array:");
    let count = loop {
        let c = read_input();
        if c > 0 {
            break c as usize;
        }

        println!("...your capacity should be more than 0. Try again:");
    };

    println!("Enter every num:");

    let mut numbers: Vec<i32> = Vec::with_capacity(count);

    for _ in 0..count{
        numbers.push(read_input());
    }

    numbers

}

fn second_largest(nums: &[i32]) -> Option<i32>{
    if nums.len() < 2 {return  None};

    let mut first_max = i32::MIN;
    let mut second_max = i32::MIN;

    for &x in nums {
        if x > first_max {
            second_max = first_max;
            first_max = x;
        } else  if x > second_max && x <= first_max {
            second_max = x;
        }
    }

    if second_max == i32::MIN { None} else { Some(second_max)}

}




fn main(){

    let nums = read_numbers();
    println!("------");
    
    match second_largest(&nums){
        Some(second_max) => println!("Second largest: {}", second_max),
        None => println!("...your array is too short."),
    }
}


#[cfg(test)]

mod tests{
    use super::*;

    #[test]
    fn test_second_largest(){
        assert_eq!(second_largest(&[1]), None);
        assert_eq!(second_largest(&[1,2]), Some(1));
        assert_eq!(second_largest(&[1,1]), Some(1));
        assert_eq!(second_largest(&[2,1]), Some(1));
        assert_eq!(second_largest(&[1,2,3]), Some(2));
        assert_eq!(second_largest(&[3,2,1]), Some(2));
        assert_eq!(second_largest(&[1,1,1]), Some(1));
    }
}