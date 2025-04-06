use std::sync::Arc;

use crate::domain::interfaces::AuthService;

#[derive(Clone)]
pub struct CheckAdminUseCase {
    auth_service: Arc<dyn AuthService>,
}

impl CheckAdminUseCase {
    pub fn new(auth_service: Arc<dyn AuthService>) -> Self {
        Self{ auth_service }
    }

    pub async fn execute(self, user_id: i64) -> Result<bool, CheckAdminError> {
        Ok(self.auth_service.is_admin(user_id).await.unwrap_or(false))
    }
}

#[derive(thiserror::Error, Debug)]
pub enum CheckAdminError {
    #[error("external service error: {0}")]
    ServiceError(#[from] Box<dyn std::error::Error + Send + Sync>),
}
