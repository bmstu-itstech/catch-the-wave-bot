use std::sync::Arc;

use crate::domain::error::DomainError;
use crate::domain::interfaces::UserRepository;
use crate::domain::models::{Profile, User};


#[derive(Clone)]
pub struct CompleteRegistrationUseCase {
    user_repo: Arc<dyn UserRepository>,
}

impl CompleteRegistrationUseCase {
    pub fn new(user_repo: Arc<dyn UserRepository>) -> Self {
        Self { user_repo }
    }

    pub async fn execute(
        &self, 
        user_id: i64,
        full_name: &str,
        group_name: &str,
    ) -> Result<User, DomainError> {
        let profile = Profile::new(full_name, group_name);
        let mut user = self.user_repo.user(user_id.into()).await?;
        user.set_profile(profile);
        self.user_repo.update(&user).await?;
        Ok(user)
    }
}
