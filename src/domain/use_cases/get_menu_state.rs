use std::sync::Arc;

use crate::domain::error::DomainError;
use crate::domain::interfaces::UserRepository;
use crate::domain::models::NextTaskStatus;

#[derive(PartialEq)]
pub enum MenuCategory {
    Profile,
    Rules,
    UserTask,
    NextTask,
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
        let user = self.user_repo.user(user_id.into()).await?;

        let mut categories = Vec::new();
        categories.push(MenuCategory::Profile);
        categories.push(MenuCategory::Rules);
        
        if user.user_task().is_some() {
            categories.push(MenuCategory::UserTask);
        }
        
        if user.next_task_status() == NextTaskStatus::Pending {
            categories.push(MenuCategory::NextTask);
        }
        
        Ok(MenuState { categories })
    }
}
