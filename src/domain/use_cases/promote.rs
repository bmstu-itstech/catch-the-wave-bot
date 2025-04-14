use std::sync::Arc;

use crate::domain::error::DomainError;
use crate::domain::interfaces::{TaskRepository, UserRepository, WeekService};


#[derive(Clone)]
pub struct AssignPartnerUseCase {
    user_repo: Arc<dyn UserRepository>,
    task_repo: Arc<dyn TaskRepository>,
    week_service: Arc<dyn WeekService>,
}

impl AssignPartnerUseCase {
    pub fn new(
        user_repo: Arc<dyn UserRepository>,
        task_repo: Arc<dyn TaskRepository>,
        week_service: Arc<dyn WeekService>,
    ) -> Self {
        Self { user_repo, task_repo, week_service }
    }

    pub async fn execute(self, user1_id: i64, user2_id: i64) -> Result<(), DomainError> {
        let current_week = self.week_service.current();
        let next_week_id = self.week_service.next(current_week);
        
        let task = match self.task_repo.task(next_week_id).await {
            Ok(task) => task,
            Err(DomainError::TaskNotFound(_)) => return Err(DomainError::NoNextTask),
            Err(e) => return Err(e),
        };
        
        let mut user1 = self.user_repo.user(user1_id.into()).await?;
        let mut user2 = self.user_repo.user(user2_id.into()).await?;
        
        user1.promote(user2.id(), task.id())?;
        user2.promote(user1.id(), task.id())?;
        
        self.user_repo.update(&user1).await?;
        self.user_repo.update(&user2).await?;
        
        Ok(())
    }
}
