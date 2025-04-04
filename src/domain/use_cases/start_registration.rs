use std::sync::Arc;

use crate::domain::interfaces::UserRepository;
use crate::domain::models;

#[derive(Clone)]
pub struct StartRegistrationUseCase {
    user_repo: Arc<dyn UserRepository>,
}

impl StartRegistrationUseCase {
    pub fn new(user_repo: Arc<dyn UserRepository>) -> Self {
        Self { user_repo }
    }
    
    pub async fn execute(
        &self, 
        user_id: i64, 
        username: String
    ) -> Result<models::User, StartRegistrationError> {
         if self.user_repo.user(user_id).await?.is_some() { 
             return Err(StartRegistrationError::UserAlreadyRegistered(user_id));
        }
        
        let user = models::User::new(user_id, username);
        self.user_repo.save(user).await
            .map_err(|e| e.into())
    }
}

#[derive(thiserror::Error, Debug)]
pub enum StartRegistrationError {
    #[error("user {0} already started registration")]
    UserAlreadyRegistered(i64),
    
    #[error("external service error: {0}")]
    ServiceError(#[from] Box<dyn std::error::Error + Send + Sync>),
}
