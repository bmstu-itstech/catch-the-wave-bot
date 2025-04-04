use std::collections::HashMap;
use std::sync::RwLock;

use crate::domain::interfaces::{QuestRepository, StdError};
use crate::domain::models::Quest;

#[derive(Default)]
pub struct InMemoryQuestRepository {
    m: RwLock<HashMap<i64, Quest>>,
}

#[async_trait::async_trait]
impl QuestRepository for InMemoryQuestRepository {
    async fn save(&self, quest: Quest) -> Result<Quest, StdError> {
        let mut guard = self.m.write().unwrap();
        guard.insert(quest.id, quest.clone());
        Ok(quest)
    }

    async fn quest(&self, id: i64) -> Result<Option<Quest>, StdError> {
        let guard = self.m.read().unwrap();
        Ok(guard.get(&id).cloned())
    }
}
