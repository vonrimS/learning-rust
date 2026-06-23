use reverse_word::*;

fn main() {
    let s = read_input();
    let res = reverse_words(&s);
    println!("{}", res);
}