use std::fmt::Display;
use crate::domain::error::DomainError;

type PartnerId = i64;

#[derive(Debug)]
#[derive(Clone)]
#[derive(PartialEq)]
pub enum NextMeetingState {
    Pending,
    Rejected,
    Accepted,
    Scheduled(PartnerId),
}

impl Display for NextMeetingState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NextMeetingState::Pending => f.write_str("pending"),
            NextMeetingState::Rejected => f.write_str("rejected"),
            NextMeetingState::Accepted => f.write_str("accepted"),
            NextMeetingState::Scheduled(id) => f.write_str(format!("scheduled:{id}").as_str()),
        }
    }
}

#[derive(Debug)]
#[derive(Clone)]
#[derive(PartialEq)]
pub struct NextMeeting {
    pub user_id: i64,
    pub quest_id: i64,
    pub state: NextMeetingState,
}

impl NextMeeting {
    pub fn new(user_id: i64, quest_id: i64) -> Self {
        Self{ user_id, quest_id, state: NextMeetingState::Pending }
    }
    
    pub fn accept(&mut self) -> Result<(), DomainError> {
        match self.state {
            NextMeetingState::Pending => {
                self.state = NextMeetingState::Accepted;
                Ok(())
            }
            _ => Err(DomainError::InvalidStatusChange(format!(
                "invalid status change for next meeting: {:?} -> {:?}", self.state, NextMeetingState::Accepted,
            )))
        }
    }
    
    pub fn reject(&mut self) -> Result<(), DomainError> {
        match self.state {
            NextMeetingState::Pending => {
                self.state = NextMeetingState::Rejected;
                Ok(())
            }
            _ => Err(DomainError::InvalidStatusChange(format!(
                "invalid status change for next meeting: {:?} -> {:?}", self.state, NextMeetingState::Rejected,
            )))
        }
    }
    
    pub fn assign_partner(&mut self, partner_id: PartnerId) -> Result<(), DomainError> {
        match self.state {
            NextMeetingState::Accepted => {
                self.state = NextMeetingState::Scheduled(partner_id);
                Ok(())
            }
            _ => Err(DomainError::InvalidStatusChange(format!(
                "invalid status change for next meeting: {:?} -> {:?}", self.state, NextMeetingState::Scheduled(partner_id),
            )))
        }
    }
}
