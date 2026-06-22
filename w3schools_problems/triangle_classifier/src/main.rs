use triangle_classifier::*;

fn main(){
    let my_triangle = create_triangle();

    if is_valid(&my_triangle) {
        println!("{}", classify(&my_triangle));
    } else {
        println!("...not a trianlge");
    }
    
}