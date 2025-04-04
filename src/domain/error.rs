#[derive(Debug, thiserror::Error)]
pub enum DomainError {
    #[error("already exists")]
    AlreadyExists(String),
    
    #[error("invalid input")]
    InvalidInput(String),
    
    #[error("invalid status change")]
    InvalidStatusChange(String),
    
    #[error("no quest found")]
    NoQuestFound,
}
