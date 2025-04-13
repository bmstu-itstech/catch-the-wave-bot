use std::sync::Arc;

use crate::domain::error::DomainError;
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

    pub async fn execute(self, user_id: i64) -> Result<CurrentMeetingDTO, DomainError> {
        let user = self.user_repo.user(user_id).await?;
        
        let current = user.current_meeting
            .ok_or(DomainError::NoCurrentMeeting(user_id))?;
        
        let partner = self.user_repo.user(current.partner_id).await?;
        
        let current_quest = self.quest_repo.quest(user.quest_index).await?;

        let dto = CurrentMeetingDTO{
            partner_username: partner.username,
            quest_text: current_quest.text,
        };
        
        Ok(dto)
    }
}
