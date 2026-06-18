use std::io;
use rust_decimal::Decimal;

// The first line of input is a count (how many scores will follow).
// The next lines each have one score.
// Calculate the average and print it with one decimal place:
// Average: [result]

fn main() {
    println!("Enter how many persons:");
    let p = read_input();
    println!("Enter score, person by person:");
    let v = create_vector(p);

    let res = calculate_avg(p, v);
    
    match res {
        Some(avg) => println!("Average: {:.2}", avg),
        None => println!("Average: N/A")
    }
}


fn read_input() -> u32{
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("...cannot read your input");

        if let Ok(num) = input.trim().parse() {
            break num;
        }
        println!("...invalid input. Try again:");
    }
}

fn create_vector(mut n: u32) -> Vec<Decimal> {
    let mut res = vec![];
    
    while n > 0 {
        res.push(Decimal::from(read_input()));
        n -= 1;
    }

    res
}

fn calculate_avg(n: u32, v: Vec<Decimal>) -> Option<Decimal>{
    if n == 0 || v.is_empty() {
        return None;
    }
    
    let mut sum: Decimal = Decimal::ZERO;
    for el in v {
        sum += el;
    }

    Some(sum / Decimal::from(n)) 
}



#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn test_calculate_avg_ok(){
        let res = calculate_avg(1,vec![Decimal::from(10)]);
        assert_eq!(res, Some(Decimal::from(10)));
    }

    #[test]
    fn test_calculate_avg_nok(){
        let res = calculate_avg(0, vec![]);
        assert_eq!(res, None);
    }
}

