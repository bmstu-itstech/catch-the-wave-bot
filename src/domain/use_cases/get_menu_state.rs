
use std::sync::Arc;

use crate::domain::interfaces::UserRepository;
use crate::domain::models::NextMeetingState;

#[derive(PartialEq)]
pub enum MenuCategory {
    Profile,
    Rules,
    NextMeeting,
    CurrentMeeting,
}

pub struct MenuState {
    pub categories: Vec<MenuCategory>,
}

#[derive(Clone)]
pub struct GetMenuStateUseCase {
    user_repo: Arc<dyn UserRepository>,
}

impl GetMenuStateUseCase {
    pub fn new(user_repo: Arc<dyn UserRepository>) -> Self {
        Self{ user_repo }
    }

    pub async fn execute(self, user_id: i64) -> Result<MenuState, GetMenuStateError> {
        let user = self.user_repo.user(user_id).await?
            .ok_or(GetMenuStateError::UserNotFound(user_id))?;

        let mut categories = Vec::new();
        categories.push(MenuCategory::Profile);
        categories.push(MenuCategory::Rules);
        
        if user.current_meeting.is_some() {
            categories.push(MenuCategory::CurrentMeeting);
        }
        
        if user.next_meeting.is_some() 
            && user.next_meeting.unwrap().state == NextMeetingState::Pending 
        {
            categories.push(MenuCategory::NextMeeting);
        }
        
        Ok(MenuState{ categories })
    }
}

#[derive(thiserror::Error, Debug)]
pub enum GetMenuStateError {
    #[error("user {0} not found")]
    UserNotFound(i64),
    
    #[error("external service error: {0}")]
    ServiceError(#[from] Box<dyn std::error::Error + Send + Sync>),
}
