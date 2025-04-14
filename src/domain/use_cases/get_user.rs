use std::sync::Arc;

use crate::domain::error::DomainError;
use crate::domain::interfaces::{TaskRepository, UserRepository};
use crate::domain::use_cases::FullUserDto;


#[derive(Clone)]
pub struct GetUserUseCase {
    user_repo: Arc<dyn UserRepository>,
    task_repo: Arc<dyn TaskRepository>,
}

impl GetUserUseCase {
    pub fn new(user_repo: Arc<dyn UserRepository>, task_repo: Arc<dyn TaskRepository>) -> Self {
        Self { user_repo, task_repo }
    }

    pub async fn execute(&self, user_id: i64) -> Result<FullUserDto, DomainError> {
        let user = self.user_repo.user(user_id.into()).await?;
        let user_task = user.user_task();
        
        match user_task {
            Some(user_task) => {
                let task = self.task_repo.task(user_task.task_id()).await?;
                let partner = self.user_repo.user(user_task.partner_id()).await?;
                FullUserDto::with_user_task(&user, &task, partner.username())
            }
            None => {
                FullUserDto::without_user_task(&user)
            }
        }
    }
}
