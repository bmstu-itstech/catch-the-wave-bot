use std::fmt::Display;

use crate::domain::error::DomainError;
use crate::domain::models::profile::Profile;
use crate::domain::models::{UserTask, WeekId};


#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub enum NextTaskStatus {
    #[default]
    Pending,
    Accepted,
    Rejected,
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct UserId(pub i64);

#[derive(Default, Debug, Clone)]
#[derive(PartialEq)]
pub struct User {
    id: UserId,
    username: String,

    profile: Option<Profile>,
    user_task: Option<UserTask>,
    next_task_status: NextTaskStatus,
    completed_tasks: i32,
}

impl User {
    pub fn new(telegram_id: i64, username: impl Into<String>) -> Self {
        Self {
            id: UserId(telegram_id),
            username: username.into(),
            ..Default::default()
        }
    }
    
    pub fn set_profile(&mut self, profile: Profile) {
        self.profile = Some(profile);
    }
    
    pub fn profile_completed(&self) -> bool {
        self.profile.is_some()
    }
    
    pub fn accept(&mut self) -> Result<(), DomainError> {
        if self.next_task_status != NextTaskStatus::Pending {
            return Err(DomainError::InvalidStateChange(
                format!("{:?} -> {:?}", self.next_task_status, NextTaskStatus::Accepted),
            ))
        }
        self.next_task_status = NextTaskStatus::Accepted;
        Ok(())
    }
    
    pub fn reject(&mut self) -> Result<(), DomainError> {
        if self.next_task_status != NextTaskStatus::Pending {
            return Err(DomainError::InvalidStateChange(
                format!("{:?} -> {:?}", self.next_task_status, NextTaskStatus::Rejected)
            ))
        }
        self.next_task_status = NextTaskStatus::Rejected;
        Ok(())
    }
    
    pub fn promote(&mut self, partner_id: UserId, week_id: WeekId) -> Result<(), DomainError> {
        if self.next_task_status != NextTaskStatus::Accepted {
            return Err(DomainError::InvalidStateChange(
                format!("{:?} -> {:?}", self.next_task_status, NextTaskStatus::Accepted)
            ))
        }
        self.user_task = Some(UserTask::new(week_id, partner_id));
        self.next_task_status = NextTaskStatus::default();
        Ok(())
    }
    
    pub fn complete_task(&mut self) -> Result<(), DomainError> {
        let user_task = self.user_task.as_mut().ok_or(DomainError::NoUserTask)?;
        user_task.complete()?;
        Ok(())
    }
    
    pub fn is_ready(&self) -> bool {
        self.next_task_status == NextTaskStatus::Accepted
    }

    pub fn restore(
        id: impl Into<UserId>,
        username: impl Into<String>,
        profile: Option<Profile>,
        user_task: Option<UserTask>,
        next_task_status: NextTaskStatus,
        completed_tasks: i32,
    ) -> Self {
        Self { id: id.into(), username: username.into(), profile, user_task, next_task_status, completed_tasks }
    }
    
    pub fn id(&self) -> UserId {
        self.id
    }
    
    pub fn username(&self) -> &str {
        &self.username
    }
    
    pub fn profile(&self) -> Option<&Profile> {
        self.profile.as_ref()
    }
    
    pub fn user_task(&self) -> Option<&UserTask> {
        self.user_task.as_ref()
    }
    
    pub fn next_task_status(&self) -> NextTaskStatus {
        self.next_task_status
    }
    
    pub fn completed_tasks(&self) -> i32 {
        self.completed_tasks
    }
}

impl Into<UserId> for i64 {
    fn into(self) -> UserId {
        UserId(self)
    }
}

impl Into<i64> for UserId {
    fn into(self) -> i64 {
        self.0
    }
}

impl Display for UserId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
