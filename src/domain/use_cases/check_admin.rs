use std::sync::Arc;

use crate::domain::error::DomainError;
use crate::domain::interfaces::AuthService;


#[derive(Clone)]
pub struct CheckAdminUseCase {
    auth_service: Arc<dyn AuthService>,
}

impl CheckAdminUseCase {
    pub fn new(auth_service: Arc<dyn AuthService>) -> Self {
        Self{ auth_service }
    }

    pub async fn execute(self, user_id: i64) -> Result<bool, DomainError> {
        Ok(self.auth_service.is_admin(user_id.into()).await.unwrap_or(false))
    }
}
