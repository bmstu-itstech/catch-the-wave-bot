use std::sync::Arc;

use crate::domain::interfaces::UserRepository;

#[derive(Clone)]
pub struct RejectMeetingUseCase {
    user_repo: Arc<dyn UserRepository>,
}

impl RejectMeetingUseCase {
    pub fn new(user_repo: Arc<dyn UserRepository>) -> Self {
        Self{ user_repo }
    }

    pub async fn execute(self, user_id: i64) -> Result<(), RejectMeetingError> {
        let mut user = self.user_repo.user(user_id).await?
            .ok_or(RejectMeetingError::UserNotFound(user_id))?;

        if user.next_meeting.is_none() {
            return Err(RejectMeetingError::NoCurrentMeeting);
        }

        user.next_meeting.as_mut().unwrap().reject()
            .map_err(|_| RejectMeetingError::InvalidStateChange)?;

        self.user_repo.save(user).await?;
        Ok(())
    }
}

#[derive(thiserror::Error, Debug)]
pub enum RejectMeetingError {
    #[error("user {0} not found")]
    UserNotFound(i64),

    #[error("user does not have meeting now")]
    NoCurrentMeeting,

    #[error("invalid next meeting state change")]
    InvalidStateChange,

    #[error("external service error: {0}")]
    ServiceError(#[from] Box<dyn std::error::Error + Send + Sync>),
}
