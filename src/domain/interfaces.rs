use chrono::{DateTime, Utc};
use crate::domain::models::{Quest, User};

pub(crate) type StdError = Box<dyn std::error::Error + Send + Sync>;

#[async_trait::async_trait]
pub trait UserRepository: Send + Sync {
    async fn save(&self, user: User) -> Result<User, StdError>;
    
    async fn user(&self, id: i64) -> Result<Option<User>, StdError>;
    
    async fn find_by_name(&self, name: &str) -> Result<Option<User>, StdError>;
    
    async fn all(&self) -> Result<Vec<User>, StdError>;
}

#[async_trait::async_trait]
pub trait AuthService: Send + Sync {
    async fn is_admin(&self, user_id: i64) -> Result<bool, StdError>;
}

#[async_trait::async_trait]
pub trait QuestRepository: Send + Sync {
    async fn create(&self, text: &str, start: DateTime<Utc>, end: DateTime<Utc>) -> Result<Quest, StdError>;
    
    async fn quest(&self, id: i64) -> Result<Option<Quest>, StdError>;
}
