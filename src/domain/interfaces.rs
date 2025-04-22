use crate::domain::error::DomainError;
use crate::domain::models::{Task, TaskId, User, UserId, WeekId};


#[async_trait::async_trait]
pub trait UserRepository: Send + Sync {
    async fn save(&self, user: &User) -> Result<(), DomainError>;
    
    async fn update(&self, user: &User) -> Result<(), DomainError>;
    
    async fn user(&self, id: UserId) -> Result<User, DomainError>;
    
    async fn find_user(&self, id: UserId) -> Result<Option<User>, DomainError>;
    
    async fn all(&self) -> Result<Vec<User>, DomainError>;
    
    async fn ready_users(&self) -> Result<Vec<User>, DomainError>;
    
    async fn active_users(&self) -> Result<Vec<User>, DomainError>;
}


#[async_trait::async_trait]
pub trait AuthService: Send + Sync {
    async fn is_admin(&self, user_id: UserId) -> Result<bool, DomainError>;
}

#[async_trait::async_trait]
pub trait TaskRepository: Send + Sync {
    async fn save(&self, task: &Task) -> Result<(), DomainError>;
    
    async fn task(&self, id: TaskId) -> Result<Task, DomainError>;
}

pub trait WeekService: Send + Sync {
    fn current(&self) -> WeekId;
    
    fn next(&self, week_id: WeekId) -> WeekId;
}
