use std::collections::HashMap;
use std::sync::RwLock;
use crate::domain::error::DomainError;
use crate::domain::interfaces::TaskRepository;
use crate::domain::models::{Task, TaskId};


#[derive(Default)]
pub struct InMemoryTaskRepository {
    m: RwLock<HashMap<TaskId, Task>>,
}

#[async_trait::async_trait]
impl TaskRepository for InMemoryTaskRepository {
    async fn save(&self, task: &Task) -> Result<(), DomainError> {
        let mut guard = self.m.write().unwrap();
        guard.insert(task.id(), task.clone());
        Ok(())
    }

    async fn task(&self, id: TaskId) -> Result<Task, DomainError> {
        let guard = self.m.read().unwrap();
        if let Some(task) = guard.get(&id) {
            Ok(task.clone())
        } else {
            Err(DomainError::TaskNotFound(id))
        }
    }
}
