use std::{collections::HashMap, io};

// The first line of input is a count (how many numbers will follow).
// The next lines each have one number.
// The last line is a target number.
// Count how many of the numbers equal the target and print:
// Count: [result]

fn main() {
    println!("How many numbers will follow:");
    let nums = read_input();

    println!("List numbers bellow:");
    let map = fill_hash_map(nums);

    println!("Your target number:");
    let target = read_input();

    let count = *map.get(&target).unwrap_or(&0);
    println!("Count: {count}");
}


fn fill_hash_map(mut n: i32) -> HashMap<i32, i32> {
    let mut map: HashMap<i32, i32> = HashMap::new();

    for _ in 0..n {
        *map.entry(read_input()).or_insert(0) += 1; 
    }

    map

}


fn read_input() -> i32{
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("...cannot read your input");

        if let Ok(num) = input.trim().parse() {
            break num;
        }
        println!("...invalid input. Try again:");
    }
}
