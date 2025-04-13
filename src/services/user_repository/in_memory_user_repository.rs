use std::collections::HashMap;
use std::sync::RwLock;

use crate::domain::error::DomainError;
use crate::domain::interfaces::UserRepository;
use crate::domain::models::User;

#[derive(Default)]
pub struct InMemoryUserRepository {
    m: RwLock<HashMap<i64, User>>,
}

#[async_trait::async_trait]
impl UserRepository for InMemoryUserRepository {
    async fn save(&self, user: &User) -> Result<(), DomainError> {
        let mut guard = self.m.write().unwrap();
        let prev = guard.insert(user.id, user.clone());
        if prev.is_some() {
            Err(DomainError::UserAlreadyExists(user.id))
        } else {
            Ok(())
        }
    }

    async fn update(&self, user: &User) -> Result<(), DomainError> {
        let mut guard = self.m.write().unwrap();
        guard.insert(user.id, user.clone());
        Ok(())
    }

    async fn user(&self, id: i64) -> Result<User, DomainError> {
        let guard = self.m.read().unwrap();
        let user = guard.get(&id);
        if let Some(user) = user {
            Ok(user.clone())
        } else {
            Err(DomainError::UserNotFound(id))       
        }
    }

    async fn all(&self) -> Result<Vec<User>, DomainError> {
        let guard = self.m.read().unwrap();
        Ok(guard.values().cloned().collect())
    }
}
