use std::sync::Arc;

use crate::domain::interfaces::UserRepository;
use crate::domain::models::User;

#[derive(Clone)]
pub struct FindUserByUsernameUseCase {
    user_repo: Arc<dyn UserRepository>,
}

impl FindUserByUsernameUseCase {
    pub fn new(user_repo: Arc<dyn UserRepository>) -> Self {
        Self { user_repo }
    }

    pub async fn execute(&self, username: &str) -> Result<Option<User>, FindUserByUsernameError> {
        self.user_repo.find_by_name(username).await
            .map_err(|e| e.into())
    }
}

#[derive(thiserror::Error, Debug)]
pub enum FindUserByUsernameError {
    #[error("external service error: {0}")]
    ServiceError(#[from] Box<dyn std::error::Error + Send + Sync>),
}
