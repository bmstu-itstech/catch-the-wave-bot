use crate::domain::error::DomainError;
use crate::domain::models::{NextTaskStatus, Task, User, UserTask, UserTaskState};

pub struct UserTaskDto {
    pub id: (i32, u32),
    pub partner_username: String,
    pub state: String,
    pub title: String,
    pub description: String,
}

pub struct UserDto {
    pub id: i64,
    pub username: String,
    pub full_name: Option<String>,
}

pub struct FullUserDto {
    pub id: i64,
    pub username: String,
    pub full_name: String,
    pub group_name: String,
    pub user_task: Option<UserTaskDto>,
    pub next_task_status: String,
    pub completed_quests: i32,
}


impl UserTaskDto {
    pub fn new(user_task: &UserTask, task: &Task, partner_username: &str) -> Self {
        Self {
            id: user_task.task_id().into(),
            partner_username: partner_username.to_string(),
            state: match user_task.state() {
                UserTaskState::Active => String::from("в процессе"),
                UserTaskState::Completed => String::from("завершено"),
            },
            title: task.title().to_string(),
            description: task.description().to_string(),
        }
    }
}

impl From<User> for UserDto {
    fn from(user: User) -> Self {
        Self {
            id: user.id().0,
            username: user.username().to_string(),
            full_name: user.profile().map(|p| p.full_name().to_string()),
        }
    }
}

impl FullUserDto {
    pub fn without_user_task(user: &User) -> Result<Self, DomainError> {
        let profile = user.profile().ok_or(DomainError::UserIsNotRegistered(user.id()))?;
        Ok(Self {
            id: user.id().0,
            username: user.username().to_string(),
            full_name: profile.full_name().to_string(),
            group_name: profile.group_name().to_string(),
            user_task: None,
            next_task_status: match user.next_task_status() {
                NextTaskStatus::Pending => String::from("нет ответа"),
                NextTaskStatus::Accepted => String::from("подтверждено"),
                NextTaskStatus::Rejected => String::from("отказано"),
            },
            completed_quests: user.completed_tasks(),
        })
    }
    
    pub fn with_user_task(user: &User, task: &Task, partner_username: &str) -> Result<Self, DomainError> {
        let profile = user.profile().ok_or(DomainError::UserIsNotRegistered(user.id()))?;
        Ok(Self {
            id: user.id().0,
            username: user.username().to_string(),
            full_name: profile.full_name().to_string(),
            group_name: profile.group_name().to_string(),
            user_task: user.user_task()
                .map(|ut| UserTaskDto::new(ut, task, partner_username)),
            next_task_status: match user.next_task_status() {
                NextTaskStatus::Pending => String::from("нет ответа"),
                NextTaskStatus::Accepted => String::from("подтверждено"),
                NextTaskStatus::Rejected => String::from("отказано"),
            },
            completed_quests: user.completed_tasks(),
        })
    }
}
