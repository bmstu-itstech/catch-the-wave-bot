use chrono::{DateTime, Utc};

#[derive(Clone)]
pub struct Quest {
    pub id: i64,
    pub start: DateTime<Utc>,
    pub end: DateTime<Utc>,
    pub text: String,
}

impl Quest {
    pub fn new(id: i64, start: DateTime<Utc>, end: DateTime<Utc>, text: String) -> Self {
        Quest{ id, start, end, text }
    }
}
