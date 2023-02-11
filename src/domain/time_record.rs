use chrono::{DateTime, Local};

pub struct TimeRecord {
    id: String,
    start: DateTime<Local>,
    end: DateTime<Local>,
    memo: String,
}

impl TimeRecord {
    pub fn new(memo: String, start: DateTime<Local>, end: DateTime<Local>) -> TimeRecord {
        TimeRecord {
            id: String::from(""),
            start,
            end,
            memo,
        }
    }

    pub fn new_with_id(id: String, start: DateTime<Local>, end: DateTime<Local>, memo: String) -> TimeRecord {
        TimeRecord {
            id,
            start,
            end,
            memo,
        }
    }
}