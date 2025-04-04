use std::sync::Arc;

use crate::domain::interfaces::{QuestRepository, UserRepository};

pub struct CurrentMeetingDTO {
    pub partner_username: String,
    pub quest_text: String,
}

#[derive(Clone)]
pub struct GetCurrentMeetingUseCase {
    user_repo: Arc<dyn UserRepository>,
    quest_repo: Arc<dyn QuestRepository>,
}

impl GetCurrentMeetingUseCase {
    pub fn new(
        user_repo: Arc<dyn UserRepository>,
        quest_repo: Arc<dyn QuestRepository>,
    ) -> Self {
        Self{ user_repo, quest_repo }
    }

    pub async fn execute(self, user_id: i64) -> Result<CurrentMeetingDTO, GetCurrentMeetingError> {
        let user = self.user_repo.user(user_id).await?
            .ok_or(GetCurrentMeetingError::UserNotFound(user_id))?;
        
        let current = user.current_meeting
            .ok_or(GetCurrentMeetingError::NoCurrentMeeting)?;
        
        let partner = self.user_repo.user(current.partner_id).await?
            .ok_or(GetCurrentMeetingError::PartnerNotFound(current.partner_id))?;
        
        let quest = self.quest_repo.quest(current.quest_id).await?
            .ok_or(GetCurrentMeetingError::QuestNotFound(current.quest_id))?;

        let dto = CurrentMeetingDTO{
            partner_username: partner.username,
            quest_text: quest.text,
        };
        
        Ok(dto)
    }
}

#[derive(thiserror::Error, Debug)]
pub enum GetCurrentMeetingError {
    #[error("user {0} not found")]
    UserNotFound(i64),
    
    #[error("partner user {0} not found")]
    PartnerNotFound(i64),
    
    #[error("no current meeting")]
    NoCurrentMeeting,
    
    #[error("quest {0} not found")]
    QuestNotFound(i64),

    #[error("external service error: {0}")]
    ServiceError(#[from] Box<dyn std::error::Error + Send + Sync>),
}
