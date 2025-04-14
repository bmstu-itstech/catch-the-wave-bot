use std::sync::Arc;

use crate::domain::error::DomainError;
use crate::domain::interfaces::TaskRepository;
use crate::domain::models::{Task, TaskId};


#[derive(Clone)]
pub struct CreateQuestUseCase {
    task_repo: Arc<dyn TaskRepository>,
}

impl CreateQuestUseCase {
    pub fn new(task_repo: Arc<dyn TaskRepository>) -> Self {
        Self { task_repo }
    }

    pub async fn execute(
        &self,
        year: i32,
        week: u32,
        title: &str,
        text: &str,
    ) -> Result<(), DomainError> {
        let id = TaskId::new(year, week);
        let task = Task::new(id, title, text);
        self.task_repo.save(&task).await?;
        Ok(())
    }
}
