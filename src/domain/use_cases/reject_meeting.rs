use std::sync::Arc;

use crate::domain::error::DomainError;
use crate::domain::interfaces::UserRepository;


#[derive(Clone)]
pub struct RejectMeetingUseCase {
    user_repo: Arc<dyn UserRepository>,
}

impl RejectMeetingUseCase {
    pub fn new(user_repo: Arc<dyn UserRepository>) -> Self {
        Self{ user_repo }
    }

    pub async fn execute(self, user_id: i64) -> Result<(), DomainError> {
        let mut user = self.user_repo.user(user_id).await?;
        user.reject()?;
        self.user_repo.update(&user).await?;
        Ok(())
    }
}
