use std::collections::HashMap;
use std::sync::{Arc, RwLock};

use crate::domain::error::DomainError;
use crate::domain::interfaces::UserRepository;
use crate::domain::models;

#[derive(Default)]
pub struct InMemoryUserRepository {
    m: Arc<RwLock<HashMap<i64, models::User>>>,
}

#[async_trait::async_trait]
impl UserRepository for InMemoryUserRepository {
    async fn save(&self, user: models::User) -> Result<models::User, DomainError> {
        let mut guard = self.m.write().unwrap();
        let telegram_id = user.telegram_id;
        match guard.insert(telegram_id, user.clone()) {
            Some(_) => Err(DomainError::AlreadyExists(format!("user already exists: {telegram_id}"))),
            None => Ok(user),
        }
    }

    async fn with_user<'a>(
        &'a self,
        telegram_id: i64,
        update_fn: Box<dyn FnOnce(models::User) -> Result<models::User, DomainError> + Send + 'a>,
    ) -> Result<models::User, DomainError> {
        let mut guard = self.m.write().unwrap();

        let user = guard.get(&telegram_id).ok_or(
            DomainError::NotFound(format!("user not found: {}", telegram_id))
        )?;

        let user = update_fn(user.clone())?;
        
        guard.insert(telegram_id, user.clone());
        Ok(user)
    }

    async fn user(&self, telegram_id: i64) -> Result<models::User, DomainError> {
        let guard = self.m.read().unwrap();
        let user = guard.get(&telegram_id).ok_or(
            DomainError::NotFound(format!("user not found: {}", telegram_id))
        )?;
        Ok(user.clone())
    }
}

#[cfg(test)]
mod tests {
    use crate::services::user_repository::test;
    use super::*;
    
    #[tokio::test]
    async fn test_in_memory_user_repository_save_once() {
        let repo = InMemoryUserRepository::default();
        test::test_user_repository_save_once(repo).await;
    }

    #[tokio::test]
    async fn test_in_memory_user_repository_save_twice() {
        let repo = InMemoryUserRepository::default();
        test::test_user_repository_save_twice(repo).await;
    }

    #[tokio::test]
    async fn test_in_memory_user_repository_get_not_found() {
        let repo = InMemoryUserRepository::default();
        test::test_user_repository_get_not_found(repo).await;
    }
}
