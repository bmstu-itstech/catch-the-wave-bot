use std::sync::Arc;
use crate::domain::error::DomainError;
use crate::domain::interfaces::{TaskRepository, WeekService};

#[derive(Clone)]
pub struct CheckNextTaskUseCase {
    task_repo: Arc<dyn TaskRepository>,
    week_service: Arc<dyn WeekService>,
}

impl CheckNextTaskUseCase {
    pub fn new(task_repo: Arc<dyn TaskRepository>, week_service: Arc<dyn WeekService>) -> Self {
        Self { task_repo, week_service }
    }
    
    pub async fn execute(&self) -> Result<bool, DomainError> {
        let current_week = self.week_service.current();
        let next_week = self.week_service.next(current_week);
        match self.task_repo.task(next_week).await {
            Ok(_) => Ok(true),
            Err(DomainError::TaskNotFound(_)) => Ok(false),
            Err(e) => Err(e),
        }
    }
}
