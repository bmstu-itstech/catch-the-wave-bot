use std::sync::Arc;
use chrono::{DateTime, Utc};

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
        start: DateTime<Utc>,
        end: DateTime<Utc>,
    ) -> Result<Quest, CreateQuestError> {
        self.quest_repo.create(text, start, end).await
            .map_err(|e| e.into())
    }
}

#[derive(thiserror::Error, Debug)]
pub enum CreateQuestError {
    #[error("external service error: {0}")]
    ServiceError(#[from] Box<dyn std::error::Error + Send + Sync>),
}
