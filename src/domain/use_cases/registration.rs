use std::sync::Arc;

use crate::domain::{interfaces, models};
use crate::domain::error::DomainError;

#[derive(Clone)]
pub struct RegistrationUseCase {
    user_repo: Arc<dyn interfaces::UserRepository>,
} 

impl RegistrationUseCase {
    pub fn new(user_repo: Arc<dyn interfaces::UserRepository>) -> Self {
        Self { user_repo }
    }
    
    pub async fn start_registration(
        &self, 
        telegram_id: i64, 
        username: Option<String>
    ) -> Result<models::User, DomainError> {
        let user = models::User::new(telegram_id, username);
        self.user_repo.save(user).await
    }
    
    pub async fn complete_registration(
        &self,
        telegram_id: i64,
        full_name: String,
        group: String,
    ) -> Result<models::User, DomainError> {
        let profile = models::Profile::new(full_name, group);
        self.user_repo.with_user(telegram_id, Box::new(|mut user| {
            user.profile = Some(profile);
            Ok(user)
        })).await
    }
}
