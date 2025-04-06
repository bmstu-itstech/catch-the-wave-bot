use std::sync::Arc;

use crate::domain::interfaces::UserRepository;
use crate::domain::models;

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
    ) -> Result<models::User, CompleteRegistrationError> {
        let profile = models::Profile::new(full_name, group_name);
        let mut user = self.user_repo.user(user_id).await?
            .ok_or(CompleteRegistrationError::UserNotFound(user_id))?;
        
        user.set_profile(profile);
        
        user.current_meeting = Some(models::CurrentMeeting::new(1, 1));
        user.next_meeting = Some(models::NextMeeting::new(1, 1));
        
        self.user_repo.save(user).await
            .map_err(|e| e.into())
    }
}

#[derive(thiserror::Error, Debug)]
pub enum CompleteRegistrationError {
    #[error("user {0} not found")]
    UserNotFound(i64),
    
    #[error("external service error: {0}")]
    ServiceError(#[from] Box<dyn std::error::Error + Send + Sync>),
}
