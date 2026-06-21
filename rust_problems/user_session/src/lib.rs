use std::time::{SystemTime, UNIX_EPOCH, Duration};

#[derive(Debug, PartialEq)]
pub struct UserSession{
    pub id: u64,
    pub username: String,
    pub ip_address: String,
    pub last_login: u64
}

pub fn update_ip_address(ip_address: String, session: UserSession) -> UserSession {
    UserSession { 
        ip_address: ip_address, 
        last_login: current_timestamp(),
        ..session 
    }
}

pub fn current_timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs()
}


#[cfg(test)]

mod tests{
    use std::thread;
    use super::*;

    #[test]
    fn test_update_ip_address(){
        let current_session = UserSession{
            id: 1,
            username: "test".to_string(),
            ip_address: "255.0.0.0".to_string(),
            last_login: current_timestamp()
        };

        thread::sleep(Duration::from_secs(5));

        let updated_session = update_ip_address("255.0.0.1".to_string(), current_session);

        assert_eq!(updated_session.id, 1);
        assert_eq!(updated_session.username, "test".to_string());
        assert_eq!(updated_session.ip_address, "255.0.0.1".to_string());
    }
}