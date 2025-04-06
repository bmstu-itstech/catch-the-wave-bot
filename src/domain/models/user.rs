use chrono::{DateTime, Utc};
use crate::domain::models::{CurrentMeeting, NextMeeting};
use crate::domain::models::profile::Profile;

#[derive(Debug)]
#[derive(Clone)]
#[derive(PartialEq)]
pub struct User {
    pub id: i64,
    pub username: String,
    pub created_at: DateTime<Utc>,

    pub profile: Option<Profile>,
    pub current_meeting: Option<CurrentMeeting>,
    pub next_meeting: Option<NextMeeting>,
}

impl User {
    pub fn new(telegram_id: i64, username: impl Into<String>) -> Self {
        Self {
            id: telegram_id,
            username: username.into(),
            created_at: Utc::now(),
            profile: None,
            current_meeting: None,
            next_meeting: None,
        }
    }

    pub fn set_profile(&mut self, profile: Profile) {
        self.profile = Some(profile);
    }
}
