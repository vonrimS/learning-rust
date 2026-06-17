use std::io;

// Read a word from input.
// Print the first and last character:
// First: [character]
// Last: [character]

fn main() {
    println!("Hello, world!");

    let input = read_input();
    println!("First: {}", read_first(&input));
    println!("Last: {}", read_last(&input));
}

fn read_input() -> String{
    loop{
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("...cannot read your input");

        let trim = input.trim();
        
        if trim.len() > 0 {
            break trim.to_string();
        } else {
            println!("...invalid input. Try again:")
        }
    }
}

fn read_first(s: &str) -> char{
    let mut chars = s.chars();
    chars.next().unwrap()
}

fn read_last(s: &str) -> char{
    let mut chars = s.chars();
    chars.next_back().unwrap()
}


#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn read_first_char(){
        assert!(read_first("abc") == 'a');
    }

    #[test]
    fn read_last_char(){
        assert!(read_last("abc") == 'c');
    }
}