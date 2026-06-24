use count_vowels::*;

fn main(){
    let phrase = read_input();
    let vowels_count = count_vowels(&phrase);
    println!("Vowels: {}", vowels_count);
}