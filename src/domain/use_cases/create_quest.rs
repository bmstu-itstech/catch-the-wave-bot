use std::sync::Arc;
use crate::domain::error::DomainError;
use crate::domain::interfaces::QuestRepository;
use crate::domain::models::Quest;


#[derive(Clone)]
pub struct CreateQuestUseCase {
    quest_repo: Arc<dyn QuestRepository>,
}

impl CreateQuestUseCase {
    pub fn new(quest_repo: Arc<dyn QuestRepository>) -> Self {
        Self { quest_repo }
    }

    pub async fn execute(
        &self,
        text: &str,
    ) -> Result<Quest, DomainError> {
        self.quest_repo.create(text).await
    }
}
