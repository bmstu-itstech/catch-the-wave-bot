use std::sync::Arc;

use crate::domain::interfaces::UserRepository;
use crate::domain::models::NextMeeting;

#[derive(Clone)]
pub struct GetNextMeetingUseCase {
    user_repo: Arc<dyn UserRepository>,
}

impl GetNextMeetingUseCase {
    pub fn new(user_repo: Arc<dyn UserRepository>) -> Self {
        Self{ user_repo }
    }

    pub async fn execute(self, user_id: i64) -> Result<Option<NextMeeting>, GetNextMeetingError> {
        let user = self.user_repo.user(user_id).await?
            .ok_or(GetNextMeetingError::UserNotFound(user_id))?;

        Ok(user.next_meeting)
    }
}

#[derive(thiserror::Error, Debug)]
pub enum GetNextMeetingError {
    #[error("user {0} not found")]
    UserNotFound(i64),

    #[error("external service error: {0}")]
    ServiceError(#[from] Box<dyn std::error::Error + Send + Sync>),
}
