use crate::domain::error::DomainError;
use crate::domain::models;

#[async_trait::async_trait]
pub trait UserRepository: Send + Sync {
    async fn save(&self, user: models::User) -> Result<models::User, DomainError>;

    async fn with_user<'a>(
        &'a self,
        telegram_id: i64,
        update_fn: Box<dyn FnOnce(models::User) -> Result<models::User, DomainError> + Send + 'a>,
    ) -> Result<models::User, DomainError>;

    async fn user(&self, telegram_id: i64) -> Result<models::User, DomainError>;
}
