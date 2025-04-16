use crate::domain::models::WeekId;

pub type TaskId = WeekId;

#[derive(Clone, Debug, PartialEq)]
pub struct Task {
    id: TaskId,
    title: String,
    description: String,
}

impl Task {
    pub fn new(task_id: impl Into<TaskId>, title: impl Into<String>, description: impl Into<String>) -> Self {
        Self { id: task_id.into(), title: title.into(), description: description.into() }
    }
    
    pub fn id(&self) -> TaskId {
        self.id
    }
    
    pub fn title(&self) -> &str {
        &self.title
    }
    
    pub fn description(&self) -> &str {
        &self.description
    }
}
