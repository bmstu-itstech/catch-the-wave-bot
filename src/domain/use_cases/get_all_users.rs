use std::sync::Arc;

use crate::domain::interfaces::UserRepository;
use crate::domain::models::User;

#[derive(Clone)]
pub struct GetAllUsersUseCase {
    user_repo: Arc<dyn UserRepository>,
}

impl GetAllUsersUseCase {
    pub fn new(user_repo: Arc<dyn UserRepository>) -> Self {
        Self { user_repo }
    }

    pub async fn execute(&self) -> Result<Vec<User>, GetAllUsersError> {
        self.user_repo.all().await
            .map_err(|e| e.into())
    }
}

#[derive(thiserror::Error, Debug)]
pub enum GetAllUsersError {
    #[error("external service error: {0}")]
    ServiceError(#[from] Box<dyn std::error::Error + Send + Sync>),
}
