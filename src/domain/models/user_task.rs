use crate::domain::error::DomainError;
use crate::domain::models::{TaskId, UserId};

#[derive(Debug, Clone, PartialEq)]
pub enum UserTaskState {
    Active,
    Completed,
}

#[derive(Debug, Clone, PartialEq)]
pub struct UserTask {
    task_id: TaskId,
    partner_id: UserId,
    state: UserTaskState,
}

impl UserTask {
    pub fn new(task_id: impl Into<TaskId>, partner_id: impl Into<UserId>) -> Self {
        Self { task_id: task_id.into(), partner_id: partner_id.into(), state: UserTaskState::Active }
    }

    pub fn is_completed(&self) -> bool {
        self.state == UserTaskState::Completed
    }

    pub fn complete(&mut self) -> Result<(), DomainError> {
        match self.state {
            UserTaskState::Active => {
                self.state = UserTaskState::Completed;
                Ok(())
            },
            _  => Err(DomainError::InvalidStateChange(
                format!("{:?} -> {:?}", self.state, UserTaskState::Completed)
            )),
        }
    }
    
    pub fn restore(
        task_id: impl Into<TaskId>, 
        partner_id: impl Into<UserId>,
        state: UserTaskState,
    ) -> Self {
        Self { task_id: task_id.into(), partner_id: partner_id.into(), state }
    }
    
    pub fn task_id(&self) -> TaskId {
        self.task_id
    }
    
    pub fn partner_id(&self) -> UserId {
        self.partner_id
    }
    
    pub fn state(&self) -> &UserTaskState {
        &self.state
    }
}
