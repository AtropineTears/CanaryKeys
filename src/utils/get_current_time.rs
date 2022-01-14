use chrono::*;

pub struct Timestamp;

impl Timestamp {
    pub fn current_timestamp() -> i64 {
        return Utc::now().timestamp()
    }
}

