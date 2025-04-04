use std::sync::Arc;

use crate::domain::interfaces::UserRepository;

#[derive(Clone)]
pub struct UserRegisteredUseCase {
    user_repo: Arc<dyn UserRepository>,
}

impl UserRegisteredUseCase {
    pub fn new(user_repo: Arc<dyn UserRepository>) -> Self {
        Self { user_repo }
    }

    pub async fn execute(
        &self,
        user_id: i64,
    ) -> Result<bool, UserRegisteredError> {
        match self.user_repo.user(user_id).await? {
            Some(u) => Ok(u.profile.is_some()),
            None => Ok(false),
        }
    }
}

#[derive(thiserror::Error, Debug)]
pub enum UserRegisteredError {
    #[error("external service error: {0}")]
    ServiceError(#[from] Box<dyn std::error::Error + Send + Sync>),
}
