use palindrome_check::*;


fn main(){
    let phrase = read_input();

    match is_palindrome(&phrase){
        true => println!("It's palindrome"),
        false => println!("...not a palindrome")
    }
}