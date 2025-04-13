use crate::domain::error::DomainError;
use crate::domain::models::{Quest, User};


#[async_trait::async_trait]
pub trait UserRepository: Send + Sync {
    async fn save(&self, user: &User) -> Result<(), DomainError>;
    
    async fn update(&self, user: &User) -> Result<(), DomainError>;
    
    async fn user(&self, id: i64) -> Result<User, DomainError>;
    
    async fn all(&self) -> Result<Vec<User>, DomainError>;
}


#[async_trait::async_trait]
pub trait AuthService: Send + Sync {
    async fn is_admin(&self, user_id: i64) -> Result<bool, DomainError>;
}


#[async_trait::async_trait]
pub trait QuestRepository: Send + Sync {
    async fn create(&self, text: &str) -> Result<Quest, DomainError>;
    
    async fn quest(&self, id: i64) -> Result<Quest, DomainError>;
    
    async fn next_quest_id(&self, id: i64) -> Result<Option<i64>, DomainError>;
}
