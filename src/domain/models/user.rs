use chrono::{DateTime, Utc};

use crate::domain::error::DomainError;
use crate::domain::models::{CurrentMeeting, CurrentMeetingState, NextMeetingState};
use crate::domain::models::profile::Profile;

#[derive(Debug, Clone)]
pub struct User {
    pub id: i64,
    pub username: String,
    pub created_at: DateTime<Utc>,

    pub profile: Option<Profile>,
    pub quest_index: i64,
    pub current_meeting: Option<CurrentMeeting>,
    pub next_meeting: NextMeetingState,
}

impl User {
    pub fn new(telegram_id: i64, username: impl Into<String>) -> Self {
        Self {
            id: telegram_id,
            username: username.into(),
            created_at: Utc::now(),
            profile: None,
            quest_index: 0,
            current_meeting: None,
            next_meeting: NextMeetingState::Pending,
        }
    }

    pub fn set_profile(&mut self, profile: Profile) {
        self.profile = Some(profile);
    }
    
    pub fn accept(&mut self) -> Result<(), DomainError> {
        if !matches!(self.next_meeting, NextMeetingState::Pending) {
            return Err(DomainError::InvalidStateChange(
                format!("{:?} -> {:?}", self.next_meeting.clone(), NextMeetingState::Accepted)
            ))
        }
        self.next_meeting = NextMeetingState::Accepted;
        Ok(())
    }
    
    pub fn reject(&mut self) -> Result<(), DomainError> {
        if !matches!(self.next_meeting, NextMeetingState::Pending) {
            return Err(DomainError::InvalidStateChange(
                format!("{:?} -> {:?}", self.next_meeting.clone(), NextMeetingState::Rejected)
            ))
        }
        self.next_meeting = NextMeetingState::Rejected;
        Ok(())
    }
    
    pub fn assign_partner(&mut self, partner_id: i64) -> Result<(), DomainError> {
        if !matches!(self.next_meeting, NextMeetingState::Accepted) {
            return Err(DomainError::InvalidStateChange(
                format!("{:?} -> {:?}", self.next_meeting.clone(), CurrentMeetingState::Active)
            ))
        }
        self.current_meeting = Some(CurrentMeeting::new(CurrentMeetingState::Active, partner_id));
        Ok(())
    }
}
