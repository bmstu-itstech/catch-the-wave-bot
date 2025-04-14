use std::sync::Arc;

use crate::domain::error::DomainError;
use crate::domain::interfaces::{TaskRepository, UserRepository};
use crate::domain::use_cases::UserTaskDto;


#[derive(Clone)]
pub struct GetUserTaskUseCase {
    user_repo: Arc<dyn UserRepository>,
    task_repo: Arc<dyn TaskRepository>,
}

impl GetUserTaskUseCase {
    pub fn new(
        user_repo: Arc<dyn UserRepository>,
        task_repo: Arc<dyn TaskRepository>,
    ) -> Self {
        Self{ user_repo, task_repo }
    }

    pub async fn execute(self, user_id: i64) -> Result<UserTaskDto, DomainError> {
        let user = self.user_repo.user(user_id.into()).await?;
        let user_task = user.user_task()
            .ok_or(DomainError::NoUserTask)?;
        let task = self.task_repo.task(user_task.task_id()).await?;
        let partner = self.user_repo.user(user_task.partner_id()).await?;
        let dto = UserTaskDto::new(&user_task, &task, partner.username());
        Ok(dto)
    }
}
