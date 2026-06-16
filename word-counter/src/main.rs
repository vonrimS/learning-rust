use std::io;

// Read a sentence from input.
// Count the number of words and print it.
// Words are separated by spaces.
// [number] words

fn main() {
    let s = read_input("Enter your phrase to count words:");
    let res: Vec<_> = vector_from_string(&s);
    
    println!("{} words", count_words(res));
}

fn vector_from_string(s: &str) -> Vec<&str>{
      s.split_whitespace().collect()
}

fn count_words(words: Vec<&str>) -> i32{
    let mut count = 0;

    for _ in words{
        count += 1;
    }

    count
}

fn read_input(s: &str) -> String {
    println!("{s}");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("...cannot read your input");
    input.trim().to_string()
}


#[cfg(test)]

mod tests{
    use super::*;

    #[test]
    fn count_words_normal(){
        let count = count_words(vector_from_string("test test test"));
        assert_eq!(count, 3);
    }

    #[test]
    fn count_words_with_empty(){
        let count = count_words(vector_from_string(""));
        assert_eq!(count, 0);
    }

    #[test]
    fn count_words_with_spaces(){
        let count = count_words(vector_from_string("   "));
        assert_eq!(count, 0);
    }

}

