use crate::domain::error::DomainError;

#[derive(Debug)]
#[derive(Clone)]
#[derive(PartialEq)]
pub enum CurrentMeetingState {
    Active,
    Verified,
}

#[derive(Debug)]
#[derive(Clone)]
#[derive(PartialEq)]
pub struct CurrentMeeting {
    pub quest_id: i64,
    pub partner_id: i64,
    pub state: CurrentMeetingState,
}

impl CurrentMeeting {
    pub fn new(quest_id: i64, partner_id: i64) -> Self {
        Self{ quest_id, partner_id, state: CurrentMeetingState::Active }
    }
    
    pub fn verify(&mut self) -> Result<(), DomainError> {
        match self.state {
            CurrentMeetingState::Active => {
                self.state = CurrentMeetingState::Verified;
                Ok(())
            }
            _ => Err(DomainError::InvalidStatusChange(format!(
                "invalid status change for current meeting: {:?} -> {:?}", self.state, CurrentMeetingState::Verified,
            )))
        }
    }
}
