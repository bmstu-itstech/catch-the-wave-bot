use std::sync::Arc;

use crate::domain::error::DomainError;
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

    pub async fn execute(&self) -> Result<Vec<User>, DomainError> {
        self.user_repo.all().await
    }
}
