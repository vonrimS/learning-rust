use std::io;

// The first line of input is a count (how many numbers will follow).
// The next lines each have one number.
// Find the smallest and largest numbers and print:
// Min: [smallest]
// Max: [largest]

fn main() {
    println!("How many numbers:");
    let n = read_input();

    println!("Enter every num below:");
    let a = fill_vector(n as usize);


    let max = find_max(&a);
    let min = find_min(&a);    
   

    println!("------");
    println!("Max: {max}");
    println!("Min: {min}");
}


fn find_min(v: &[i32]) -> i32{
    *v.iter().min().unwrap()
}


fn find_max(v: &[i32]) -> i32{
    *v.iter().max().unwrap()
}

fn read_input() -> i32{
    loop{
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("...cannot read your input");

        if let Ok(num) = input.trim().parse(){
            if num > 0 {
                break num;
            }
        }

        println!("...invalid input. Try again:");
    }
}

fn fill_vector(n: usize) -> Vec<i32>{
    let mut vec = Vec::with_capacity(n);

    for _ in 0..n {
        vec.push(read_input());
    }

    vec
}


#[cfg(test)]

mod tests{
    use super::*;

    #[test]
    fn test_find_min_normal(){        
        assert_eq!(find_min(&(vec![1, 2, 3])), 1);
        assert_eq!(find_min(&(vec![0, 2, 3])), 0);
        assert_eq!(find_min(&(vec![-1, 2, 3])), -1);
    }

    #[test]
    fn test_find_max_normal(){        
        assert_eq!(find_max(&(vec![1, 2, 3])), 3);
        assert_eq!(find_max(&(vec![0, 2, 3])), 3);
        assert_eq!(find_max(&(vec![-1, 2, 3])), 3);
    }

}