use time_converter::{count_h_m_s, read_input};

fn main(){
    let time = read_input();
    let res = count_h_m_s(time);

    println!("{}h {}m {}s", res.0.to_string(), res.1.to_string(), res.2.to_string());
}