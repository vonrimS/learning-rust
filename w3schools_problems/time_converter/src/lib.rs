// Read a number of total seconds from input.
// Convert it to hours, minutes, and seconds.
// Print the result like this:
//      [h]h [m]m [s]s

use std::io;

pub fn read_input() -> u32{
    println!("Enter your time (in seconds):");
    let mut input = String::new();

    loop {
        input.clear();

        io::stdin().read_line(&mut input).expect("...cannot read your input");

        if let Ok(time) = input.trim().parse(){
            if time > 0 {
                break time;
            }
        }

        println!("...invalid input. Try again:");
    }
}

pub fn count_units(input: u32, unit: u32) -> (u32, u32){
    let unit_qnt = input/unit;
    let rem = input % unit;
    (unit_qnt, rem)
}

pub fn count_h_m_s(input: u32) -> (u32, u32, u32){
    let h = count_units(input, 3600);
    let m = count_units(h.1, 60);

    (h.0, m.0, m.1)
}



#[cfg(test)]

mod tests{
    use super::*;

    #[test]
    fn test_count_h_m_s(){
        assert_eq!(count_h_m_s(8611_u32), (2,23,31));
        assert_eq!(count_h_m_s(3661_u32), (1,1,1));
        assert_eq!(count_h_m_s(661_u32), (0,11,1));
        assert_eq!(count_h_m_s(61_u32), (0,1,1));
    }
}