// Create a UserSession struct with fields: id: u64, username: String, ip_address: String, and last_login: u64. 
// Write a function that takes an existing session and a new IP address, 
// and returns a new session where only the IP address and last_login are updated, 
// while all other fields are moved from the old structure without manual field-by-field copying.

use std::thread;
use std::time::Duration;
use user_session::{update_ip_address, current_timestamp, UserSession };


fn main(){
    let session = UserSession {
        id: 1,
        username: "test".to_string(),
        ip_address: "255.0.0.0".to_string(),
        last_login: current_timestamp(),
    };

    println!("{:?}", session);

    thread::sleep(Duration::from_secs(5));
    
    let session_updated = update_ip_address("255.0.0.1".to_string(), session);
    
    println!("{:?}", session_updated);
    
}

