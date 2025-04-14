use std::sync::Arc;

use crate::domain::error::DomainError;
use crate::domain::interfaces::{TaskRepository, WeekService};
use crate::domain::models::Task;


#[derive(Clone)]
pub struct CreateNextTaskUseCase {
    task_repo: Arc<dyn TaskRepository>,
    week_service: Arc<dyn WeekService>,
}

impl CreateNextTaskUseCase {
    pub fn new(task_repo: Arc<dyn TaskRepository>, week_service: Arc<dyn WeekService>) -> Self {
        Self { task_repo, week_service }
    }

    pub async fn execute(
        &self,
        title: &str,
        text: &str,
    ) -> Result<(), DomainError> {
        let current_week = self.week_service.current();
        let next_week = self.week_service.next(current_week);
        let task = Task::new(next_week, title, text);
        self.task_repo.save(&task).await?;
        Ok(())
    }
}
