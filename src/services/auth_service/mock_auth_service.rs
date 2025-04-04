use crate::domain::interfaces::{AuthService, StdError};

pub struct MockAuthService {
    pub admin_ids: Vec<i64>,
}

#[async_trait::async_trait]
impl AuthService for MockAuthService {
    async fn is_admin(&self, user_id: i64) -> Result<bool, StdError> {
        Ok(self.admin_ids.contains(&user_id))
    }
}
