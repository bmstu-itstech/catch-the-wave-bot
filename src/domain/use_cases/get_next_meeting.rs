use std::sync::Arc;

use crate::domain::error::DomainError;
use crate::domain::interfaces::{QuestRepository, UserRepository};
use crate::domain::models::NextMeetingState;



pub struct NextMeetingDTO {
    pub text: String,
    pub state: NextMeetingState,
}

#[derive(Clone)]
pub struct GetNextMeetingUseCase {
    user_repo: Arc<dyn UserRepository>,
    quest_repo: Arc<dyn QuestRepository>,
}

impl GetNextMeetingUseCase {
    pub fn new(user_repo: Arc<dyn UserRepository>, quest_repo: Arc<dyn QuestRepository>) -> Self {
        Self{ user_repo, quest_repo }
    }

    pub async fn execute(self, user_id: i64) -> Result<NextMeetingDTO, DomainError> {
        let user = self.user_repo.user(user_id).await?;
        let next_quest_id = self.quest_repo.next_quest_id(user.quest_index).await?.unwrap();
        let next_quest = self.quest_repo.quest(next_quest_id).await?;
        let dto = NextMeetingDTO {
            text: next_quest.text,
            state: user.next_meeting,
        };
        Ok(dto)
    }
}
