use std::collections::HashMap;
use std::sync::RwLock;

use crate::domain::error::DomainError;
use crate::domain::interfaces::UserRepository;
use crate::domain::models::{User, UserId};

#[derive(Default)]
pub struct InMemoryUserRepository {
    m: RwLock<HashMap<UserId, User>>,
}

#[async_trait::async_trait]
impl UserRepository for InMemoryUserRepository {
    async fn save(&self, user: &User) -> Result<(), DomainError> {
        let mut guard = self.m.write().unwrap();
        if guard.get(&user.id()).is_some() {
            Err(DomainError::UserAlreadyExists(user.id()))
        } else {
            guard.insert(user.id(), user.clone());
            Ok(())
        }
    }

    async fn update(&self, user: &User) -> Result<(), DomainError> {
        let mut guard = self.m.write().unwrap();
        guard.insert(user.id(), user.clone());
        Ok(())
    }

    async fn user(&self, id: UserId) -> Result<User, DomainError> {
        Ok(self.find_user(id).await?.ok_or(DomainError::UserNotFound(id))?)
    }
    
    async fn find_user(&self, id: UserId) -> Result<Option<User>, DomainError> {
        let guard = self.m.read().unwrap();
        Ok(guard.get(&id).cloned())
    }

    async fn all(&self) -> Result<Vec<User>, DomainError> {
        let guard = self.m.read().unwrap();
        Ok(guard.values().cloned().collect())
    }
    
    async fn ready_users(&self) -> Result<Vec<User>, DomainError> {
        let guard = self.m.read().unwrap();
        Ok(guard
            .values()
            .filter(|&user| user.is_ready())
            .map(|user| user.clone())
            .collect()
        )
    }
}
