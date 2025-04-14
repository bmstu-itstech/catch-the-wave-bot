use crate::domain::error::DomainError;
use crate::domain::interfaces::AuthService;
use crate::domain::models::UserId;


#[derive(Default)]
pub struct MockAuthService {
    pub admin_ids: Vec<UserId>,
}

#[async_trait::async_trait]
impl AuthService for MockAuthService {
    async fn is_admin(&self, user_id: UserId) -> Result<bool, DomainError> {
        Ok(self.admin_ids.contains(&user_id))
    }
}

impl MockAuthService {
    pub fn with_admin_ids(admin_ids: impl IntoIterator<Item = impl Into<UserId>>) -> MockAuthService {
        MockAuthService {
            admin_ids: admin_ids
                .into_iter()
                .map(|id| id.into())
                .collect::<Vec<UserId>>(),
        }
    }
}
