use chrono::{DateTime, Utc};
use crate::domain::models::profile::Profile;

#[derive(Debug)]
#[derive(Clone)]
#[derive(PartialEq)]
pub struct User {
    pub telegram_id: i64,
    pub username: Option<String>,   // Аккаунт в ТГ может иметь скрытый никнейм (отсутствующий)
    pub created_at: DateTime<Utc>,
    
    pub profile: Option<Profile>,
}

impl User {
    pub fn new(telegram_id: i64, username: Option<String>) -> Self {
        Self {
            telegram_id,
            username,
            created_at: Utc::now(),
            profile: None,
        }
    }
}
