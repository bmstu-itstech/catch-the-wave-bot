use deadpool_postgres::{GenericClient, Pool};
use tokio_postgres::Row;

use crate::domain::error::DomainError;
use crate::domain::interfaces::TaskRepository;
use crate::domain::models::{Task, TaskId};
use crate::utils::postgres::helpers;


#[derive(Clone, Debug)]
struct TaskModel {
    pub year:        i32,
    pub week:        i32,
    pub title:       String,
    pub description: String,
}

pub struct PostgresTaskRepository {
    pool: Pool,
}

impl PostgresTaskRepository {
    pub fn new(pool: Pool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl TaskRepository for PostgresTaskRepository {
    async fn save(&self, task: &Task) -> Result<(), DomainError> {
        let client = self.pool
            .get()
            .await
            .map_err(|err| DomainError::Other(err.into()))?;

        let task_model: TaskModel = task.clone().into();

        client.execute(
            r#"
            INSERT INTO tasks (
                year,
                week,
                title,
                description
            ) VALUES ($1, $2, $3, $4)
            "#,
            &[
                &task_model.year,
                &task_model.week,
                &task_model.title,
                &task_model.description,
            ],
        )
            .await
            .map_err(|err| {
                if helpers::is_unique_violation(&err) {
                    DomainError::TaskAlreadyExists(task.id())
                } else {
                    DomainError::Other(err.into())
                }
            })?;

        Ok(())
    }

    async fn task(&self, id: TaskId) -> Result<Task, DomainError> {
        let client = self.pool
            .get()
            .await
            .map_err(|err| DomainError::Other(err.into()))?;

        let row = client.query_opt(
            r#"
            SELECT
                year,
                week,
                title,
                description
            FROM tasks
            WHERE year = $1 AND week = $2
            "#,
            &[
                &id.year,
                &(id.week as i32),
            ],
        )
            .await
            .map_err(|err| DomainError::Other(err.into()))?;

        let Some(row) = row else {
            return Err(DomainError::TaskNotFound(id));
        };

        let model: TaskModel = row.into();

        Ok(model.into())
    }
}

impl From<Task> for TaskModel {
    fn from(task: Task) -> Self {
        Self {
            year: task.id().year,
            week: task.id().week as i32,
            title: task.title().to_string(),
            description: task.description().to_string(),
        }
    }
}

impl Into<Task> for TaskModel {
    fn into(self) -> Task {
        Task::new(
            TaskId::new(self.year, self.week as u32),
            self.title,
            self.description,
        )
    }
}

impl From<Row> for TaskModel {
    fn from(row: Row) -> Self {
        Self {
            year: row.get("year"),
            week: row.get("week"),
            title: row.get("title"),
            description: row.get("description"),
        }
    }
}

#[cfg(test)]
mod integration_tests {
    use rand::{random, random_range};
    use crate::utils::postgres::testing::test_db_setup;
    use super::*;


    fn create_task_from_id(id: i64) -> Task {
        Task::new(
            TaskId::new(random_range(1970..2025), random_range(1..52)),
            format!("Task #{}", id),
            format!("Task #{} description", id),
        )
    }

    #[tokio::test]
    async fn test_save_and_retrieve_task() {
        let pool = test_db_setup().await;
        let repo = PostgresTaskRepository::new(pool.clone());
        let task = create_task_from_id(random());

        repo.save(&task)
            .await
            .expect("failed to save task");

        let retrieved_task = repo.task(task.id())
            .await
            .expect("failed to retrieve task");

        assert_eq!(retrieved_task, task);
    }

    #[tokio::test]
    async fn test_task_not_exists() {
        let pool = test_db_setup().await;
        let repo = PostgresTaskRepository::new(pool.clone());
        let non_existent_id = TaskId::new(random_range(1970..2025), random_range(1..52));

        let result = repo.task(non_existent_id.clone()).await;

        match result {
            Err(DomainError::TaskNotFound(id)) => {
                assert_eq!(id, non_existent_id, "returned task ID should match the requested one");
            },
            Ok(_) => panic!("expected TaskNotFound error, but got Ok"),
            Err(e) => panic!("expected TaskNotFound error, but got different error: {:?}", e),
        }
    }

    #[tokio::test]
    async fn test_task_exists() {
        let pool = test_db_setup().await;
        let repo = PostgresTaskRepository::new(pool.clone());
        let task = create_task_from_id(random());

        repo.save(&task)
            .await
            .expect("failed to save task");

        let result = repo.save(&task)
            .await;

        match result {
            Err(DomainError::TaskAlreadyExists(id)) => {
                assert_eq!(id, task.id(), "returned task ID should match the requested one");
            },
            Ok(_) => panic!("expected TaskAlreadyExists error, but got Ok"),
            Err(e) => panic!("expected TaskAlreadyExists error, but got different error: {:?}", e),
        }
    }
}

