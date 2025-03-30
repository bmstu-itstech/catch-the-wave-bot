#[derive(Debug, thiserror::Error)]
pub enum DomainError {
    #[error("already exists")]
    AlreadyExists(String),
    
    #[error("not found")]
    NotFound(String),
    
    #[error("internal storage error")]
    StorageError(#[from] Box<dyn std::error::Error + Send + Sync>),
}

/*
impl DomainError {
    pub fn storage<E>(err: E) -> Self
    where
        E: std::error::Error + Send + Sync + 'static,
    {
        Self::StorageError(Box::new(err))
    }
}
*/
