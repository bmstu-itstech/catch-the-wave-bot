use std::sync::Arc;

use crate::domain::error::DomainError;
use crate::domain::interfaces::UserRepository;
use crate::domain::use_cases::UserDto;


#[derive(Clone)]
pub struct GetActiveUsersUseCase {
    user_repo: Arc<dyn UserRepository>,
}

impl GetActiveUsersUseCase {
    pub fn new(user_repo: Arc<dyn UserRepository>) -> Self {
        Self { user_repo }
    }

    pub async fn execute(&self) -> Result<Vec<UserDto>, DomainError> {
        Ok(self.user_repo
            .active_users().await?
            .into_iter()
            .map(UserDto::from)
            .collect())
    }
}
