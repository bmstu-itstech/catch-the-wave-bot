use std::sync::Arc;

use crate::domain::error::DomainError;
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
        username: &str,
    ) -> Result<(), DomainError> { 
        let user = models::User::new(user_id, username);
        self.user_repo.save(&user).await
    }
}
