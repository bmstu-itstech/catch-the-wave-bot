use crate::domain::models::{TaskId, UserId};

pub type StdError = Box<dyn std::error::Error + Send + Sync>;


#[derive(thiserror::Error, Debug)]
pub enum DomainError {
    #[error("user {0} not found")]
    UserNotFound(UserId),
    
    #[error("task {0} not found")]
    TaskNotFound(TaskId),
    
    #[error("user {0} already exists")]
    UserAlreadyExists(UserId),
    
    #[error("invalid status change: {0}")]
    InvalidStateChange(String),
    
    #[error("no active task")]
    NoUserTask,
    
    #[error("no next task")]
    NoNextTask,
    
    #[error("user {0} did not completed registration")]
    UserIsNotRegistered(UserId),
    
    #[error(transparent)]
    Other(#[from] StdError),
}
