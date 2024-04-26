use chrono::{FixedOffset, NaiveDateTime, Utc};

pub struct TimeHelper;

impl TimeHelper {
    pub fn now() -> NaiveDateTime {
        // Brasilia timezone (-03:00) 
        let offset = FixedOffset::west_opt(3 * 60 * 60).unwrap();
        let now = Utc::now().with_timezone(&offset).naive_local();

        return now;
    }
}