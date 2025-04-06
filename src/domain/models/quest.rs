use chrono::{DateTime, Utc};

#[derive(Clone)]
pub struct Quest {
    pub id: i64,
    pub text: String,
    pub start: DateTime<Utc>,
    pub end: DateTime<Utc>,
}

impl Quest {
    pub fn new(id: i64, text: impl Into<String>, start: DateTime<Utc>, end: DateTime<Utc>) -> Self {
        Quest{ id, text: text.into(), start, end, }
    }
}
