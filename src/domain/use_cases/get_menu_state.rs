use std::sync::Arc;

use crate::domain::error::DomainError;
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

    pub async fn execute(self, user_id: i64) -> Result<MenuState, DomainError> {
        let user = self.user_repo.user(user_id).await?;

        let mut categories = Vec::new();
        categories.push(MenuCategory::Profile);
        categories.push(MenuCategory::Rules);
        
        if user.current_meeting.is_some() {
            categories.push(MenuCategory::CurrentMeeting);
        }
        
        if matches!(user.next_meeting, NextMeetingState::Pending) {
            categories.push(MenuCategory::NextMeeting);
        }
        
        Ok(MenuState{ categories })
    }
}
