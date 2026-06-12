use text_io::read;

// Read a width and a height from input.
// Print a rectangle border using * characters.
// The first and last rows are full rows of stars. The rows in between have a star at the start and end, with spaces in the middle.

fn main() {
    println!("Enter width and height:");

    let w: i32 = read!();
    let h: i32 = read!();
 
    draw_rectangle(h, w);
} 

fn draw_rectangle(h:i32, w:i32){
    let mut rows = h;
    while rows > 0 {
        if rows == 1 || rows == h {
            println!("{}", fill_row_normal(w));
        } else {
            println!("{}", fill_row_middle(w));    
        }
        rows -= 1;
    }
}

fn fill_row_normal(mut w: i32) -> String{
    let mut res = String::new();
    while w > 0 {
        res.push_str("*");
        w -= 1;
    }
    res
}

fn fill_row_middle(mut w: i32) -> String{
    let mut res = String::new();
    let max_len = w;
    while w > 0  {
        if w == 1 || w == max_len  {
            res.push_str("*");
        } else {
            res.push_str(" ");
        }
        w -= 1;
    }
    res
}
