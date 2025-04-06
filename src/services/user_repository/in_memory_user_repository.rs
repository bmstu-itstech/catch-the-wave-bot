use std::collections::HashMap;
use std::sync::RwLock;

use crate::domain::interfaces::{StdError, UserRepository};
use crate::domain::models::User;

#[derive(Default)]
pub struct InMemoryUserRepository {
    m: RwLock<HashMap<i64, User>>,
}

#[async_trait::async_trait]
impl UserRepository for InMemoryUserRepository {
    async fn save(&self, user: User) -> Result<User, StdError> {
        let mut guard = self.m.write().unwrap();
        guard.insert(user.id, user.clone());
        Ok(user)
    }

    async fn user(&self, id: i64) -> Result<Option<User>, StdError> {
        let guard = self.m.read().unwrap();
        Ok(guard.get(&id).cloned())
    }

    async fn all(&self) -> Result<Vec<User>, StdError> {
        let guard = self.m.read().unwrap();
        Ok(guard.values().cloned().collect())
    }
}
