use std::collections::HashMap;
use std::sync::RwLock;
use chrono::{DateTime, Utc};

use crate::domain::interfaces::{QuestRepository, StdError};
use crate::domain::models::Quest;

#[derive(Default)]
pub struct InMemoryQuestRepository {
    m: RwLock<HashMap<i64, Quest>>,
}

#[async_trait::async_trait]
impl QuestRepository for InMemoryQuestRepository {
    async fn create(&self, text: &str, start: DateTime<Utc>, end: DateTime<Utc>) -> Result<Quest, StdError> {
        let mut guard = self.m.write().unwrap();
        let id = guard.len() as i64;
        let quest = Quest::new(id, text, start, end);
        guard.insert(id, quest.clone());
        Ok(quest)
    }

    async fn quest(&self, id: i64) -> Result<Option<Quest>, StdError> {
        let guard = self.m.read().unwrap();
        Ok(guard.get(&id).cloned())
    }
}
