use std::sync::Arc;

use crate::domain::error::DomainError;
use crate::domain::interfaces::UserRepository;
use crate::domain::models::User;


#[derive(Clone)]
pub struct GetUserUseCase {
    user_repo: Arc<dyn UserRepository>,
}

impl GetUserUseCase {
    pub fn new(user_repo: Arc<dyn UserRepository>) -> Self {
        Self { user_repo }
    }

    pub async fn execute(&self, user_id: i64) -> Result<User, DomainError> {
        self.user_repo.user(user_id).await
    }
}
