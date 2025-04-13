use std::collections::HashMap;
use std::sync::RwLock;

use crate::domain::error::DomainError;
use crate::domain::interfaces::QuestRepository;
use crate::domain::models::Quest;

#[derive(Default)]
pub struct InMemoryQuestRepository {
    m: RwLock<HashMap<i64, Quest>>,
}

#[async_trait::async_trait]
impl QuestRepository for InMemoryQuestRepository {
    async fn create(&self, text: &str) -> Result<Quest, DomainError> {
        let mut guard = self.m.write().unwrap();
        let id = guard.len() as i64;
        let quest = Quest::new(id, text);
        guard.insert(id, quest.clone());
        Ok(quest)
    }

    async fn quest(&self, id: i64) -> Result<Quest, DomainError> {
        let guard = self.m.read().unwrap();
        let quest = guard.get(&id);
        if let Some(quest) = quest {
            Ok(quest.clone())
        } else {
            Err(DomainError::QuestNotFound(id))
        }
    }

    async fn next_quest_id(&self, id: i64) -> Result<Option<i64>, DomainError> {
        let guard = self.m.read().unwrap();
        Ok(guard
            .keys()
            .filter(|&&i| i > id)
            .min()
            .copied())
    }
}
