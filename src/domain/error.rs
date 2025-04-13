pub type StdError = Box<dyn std::error::Error + Send + Sync>;


#[derive(thiserror::Error, Debug)]
pub enum DomainError {
    #[error("user {0} not found")]
    UserNotFound(i64),
    
    #[error("quest {0} not found")]
    QuestNotFound(i64),
    
    #[error("user {0} already exists")]
    UserAlreadyExists(i64),
    
    #[error("invalid status change: {0}")]
    InvalidStateChange(String),
    
    #[error("no current meeting for user {0}")]
    NoCurrentMeeting(i64),
    
    #[error(transparent)]
    Other(#[from] StdError),
}
