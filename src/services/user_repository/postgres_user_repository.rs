use deadpool_postgres::{Client, GenericClient, Pool};
use tokio_postgres::Row;
use postgres_types::{FromSql, ToSql};

use crate::domain::error::DomainError;
use crate::domain::interfaces::UserRepository;
use crate::domain::models::{NextTaskStatus, Profile, TaskId, User, UserId, UserTask, UserTaskState};
use crate::utils::postgres::helpers::is_unique_violation;


#[derive(Debug, Clone, ToSql, FromSql)]
#[postgres(name = "user_task_state")]
enum UserTaskStateModel {
    #[postgres(name = "active")]
    Active,

    #[postgres(name = "completed")]
    Completed,
}

#[derive(Clone, Debug)]
struct UserTaskModel {
    user_id:    i64,    // PK   FK(users)
    task_year:  i32,    //      FK (tasks)
    task_week:  i32,    //      FK (tasks)
    partner_id: i64,    //      FK (users)
    state:      UserTaskStateModel,
}

#[derive(Debug, Clone, ToSql, FromSql)]
#[postgres(name = "next_task_status")]
enum NextTaskStatusModel {
    #[postgres(name = "pending")]
    Pending,

    #[postgres(name = "accepted")]
    Accepted,

    #[postgres(name = "rejected")]
    Rejected,
}

#[derive(Clone, Debug)]
struct UserModel {
    id:                 i64,    // PK
    username:           String,
    full_name:          Option<String>,
    group_name:         Option<String>,
    next_task_status:   NextTaskStatusModel,
    completed_tasks:    i32,
}

pub struct PostgresUserRepository {
    pool: Pool,
}

impl PostgresUserRepository {
    pub fn new(pool: Pool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl UserRepository for PostgresUserRepository {
    async fn save(&self, user: &User) -> Result<(), DomainError> {
        let mut client = self.pool
            .get()
            .await
            .map_err(|err| DomainError::Other(err.into()))?;

        let tr = client
            .transaction()
            .await
            .map_err(|err| DomainError::Other(err.into()))?;

        let (user_model, user_task_model) = user.into();

        tr.execute(
            r#"
            INSERT INTO users (
                id,
                username,
                full_name,
                group_name,
                next_task_status,
                completed_tasks
            ) VALUES ($1, $2, $3, $4, $5, $6)
            "#,
            &[
                &user_model.id,
                &user_model.username,
                &user_model.full_name,
                &user_model.group_name,
                &user_model.next_task_status,
                &user_model.completed_tasks,
            ],
        )
            .await
            .map_err(|err| {
                if is_unique_violation(&err) {
                    DomainError::UserAlreadyExists(user.id())
                } else {
                    DomainError::Other(err.into())
                }
            })?;

        if let Some(ut) = user_task_model {
            tr.execute(
                r#"
                INSERT INTO user_tasks (
                    user_id,
                    task_year,
                    task_week,
                    partner_id,
                    state
                ) VALUES ($1, $2, $3, $4, $5)
                "#,
                &[
                    &ut.user_id,
                    &ut.task_year,
                    &ut.task_week,
                    &ut.partner_id,
                    &ut.state,
                ]
            )
                .await
                .map_err(|err| DomainError::Other(err.into()))?;
        }

        tr.commit()
            .await
            .map_err(|err| DomainError::Other(err.into()))?;

        Ok(())
    }

    async fn update(&self, user: &User) -> Result<(), DomainError> {
        let mut client = self.pool
            .get()
            .await
            .map_err(|err| DomainError::Other(err.into()))?;

        let tr = client
            .transaction()
            .await
            .map_err(|err| DomainError::Other(err.into()))?;

        let (user_model, user_task_model) = user.into();

        tr.execute(
            r#"
            UPDATE users
            SET
                username = $2,
                full_name = $3,
                group_name = $4,
                next_task_status = $5,
                completed_tasks = $6
            WHERE id = $1
            "#,
            &[
                &user_model.id,
                &user_model.username,
                &user_model.full_name,
                &user_model.group_name,
                &user_model.next_task_status,
                &user_model.completed_tasks,
            ],
        )
            .await
            .map_err(|err| DomainError::Other(err.into()))?;

        if let Some(ut) = user_task_model {
            tr.execute(
                r#"
                INSERT INTO user_tasks (
                    user_id,
                    task_year,
                    task_week,
                    partner_id,
                    state
                ) VALUES ($1, $2, $3, $4, $5)
                ON CONFLICT (user_id)
                    DO UPDATE
                    SET
                        task_year  = $2,
                        task_week  = $3,
                        partner_id = $4,
                        state      = $5
                "#,
                &[
                    &ut.user_id,
                    &ut.task_year,
                    &ut.task_week,
                    &ut.partner_id,
                    &ut.state,
                ]
            )
                .await
                .map_err(|err| DomainError::Other(err.into()))?;
        }

        tr.commit()
            .await
            .map_err(|err| DomainError::Other(err.into()))?;

        Ok(())
    }

    async fn user(&self, id: UserId) -> Result<User, DomainError> {
        self.find_user(id)
            .await?
            .ok_or(DomainError::UserNotFound(id))
    }

    async fn find_user(&self, id: UserId) -> Result<Option<User>, DomainError> {
        let client = self.pool
            .get()
            .await
            .map_err(|err| DomainError::Other(err.into()))?;

        let row = client
            .query_opt(
                r#"
                SELECT
                    id,
                    username,
                    full_name,
                    group_name,
                    next_task_status,
                    completed_tasks
                FROM users
                WHERE id = $1
                "#,
                &[&id.0],
            )
            .await
            .map_err(|err| DomainError::Other(err.into()))?;

        let Some(row) = row else {
            return Ok(None);
        };

        let user_model: UserModel = row.into();
        let user_task_model = Self::user_task_model(&client, user_model.id).await?;
        let user: User  = (user_model, user_task_model).into();

        Ok(Some(user))
    }

    async fn all(&self) -> Result<Vec<User>, DomainError> {
        self.fetch_users(None).await
    }

    async fn ready_users(&self) -> Result<Vec<User>, DomainError> {
        self.fetch_users(Some("next_task_status = 'accepted'")).await
    }
    
    async fn active_users(&self) -> Result<Vec<User>, DomainError> {
        let client = self.pool
            .get()
            .await
            .map_err(|err| DomainError::Other(err.into()))?;

        let rows = client
            .query(
                r#"
                SELECT
                    id,
                    username,
                    full_name,
                    group_name,
                    next_task_status,
                    completed_tasks,
                    user_id,
                    task_year,
                    task_week,
                    partner_id,
                    state
                FROM users
                RIGHT JOIN user_tasks t ON t.user_id = users.id
                WHERE state = 'active'
                "#,
                &[],
            )
                .await
                .map_err(|err| DomainError::Other(err.into()))?;
        
        let users: Vec<_> = rows
            .into_iter()
            .map(|row| {
                let user_model: UserModel = row.clone().into();
                let user_task_model: UserTaskModel = row.into();
                (user_model, Some(user_task_model)).into()
            })
            .collect();
        
        Ok(users)
    }
}

impl PostgresUserRepository {
    async fn user_task_model(client: &Client, user_id: i64) -> Result<Option<UserTaskModel>, DomainError> {
        let row = client
            .query_opt(
                r#"
                SELECT
                    user_id,
                    task_year,
                    task_week,
                    partner_id,
                    state
                FROM user_tasks
                WHERE user_id = $1
                "#,
                &[&user_id],
            )
            .await
            .map_err(|err| DomainError::Other(err.into()))?;

        Ok(row.map(|r| r.into()))
    }

    async fn fetch_users(&self, where_clause: Option<&str>) -> Result<Vec<User>, DomainError> {
        let client = self.pool
            .get()
            .await
            .map_err(|err| DomainError::Other(err.into()))?;

        let query = format!(
            r#"
            SELECT
                id,
                username,
                full_name,
                group_name,
                next_task_status,
                completed_tasks
            FROM users
            {}
            "#,
            where_clause.map(|clause| format!("WHERE {}", clause)).unwrap_or_default(),
        );

        let user_rows = client
            .query(query.as_str(), &[])
            .await
            .map_err(|err| DomainError::Other(err.into()))?;

        let mut users: Vec<User> = Vec::new();
        for user_row in user_rows {
            let user_model: UserModel = user_row.into();
            let user_task_model: Option<UserTaskModel> = Self::user_task_model(&client, user_model.id).await?;
            let user: User = (user_model, user_task_model).into();
            users.push(user);
        }

        Ok(users)
    }
}

impl From<UserTaskState> for UserTaskStateModel {
    fn from(user_task_state: UserTaskState) -> Self {
        match user_task_state {
            UserTaskState::Active => UserTaskStateModel::Active,
            UserTaskState::Completed => UserTaskStateModel::Completed,
        }
    }
}

impl From<NextTaskStatus> for NextTaskStatusModel {
    fn from(next_task_status: NextTaskStatus) -> Self {
       match next_task_status {
           NextTaskStatus::Pending => NextTaskStatusModel::Pending,
           NextTaskStatus::Accepted => NextTaskStatusModel::Accepted,
           NextTaskStatus::Rejected => NextTaskStatusModel::Rejected,
       }
    }
}

impl From<&User> for (UserModel, Option<UserTaskModel>) {
    fn from(user: &User) -> Self {
        (
            UserModel {
                id:                 user.id().0,
                username:           user.username().to_string(),
                full_name:          user.profile().map(|p| p.full_name().to_string()),
                group_name:         user.profile().map(|p| p.group_name().to_string()),
                next_task_status:   user.next_task_status().into(),
                completed_tasks:    user.completed_tasks(),
            },
            user.user_task().map(|ut| UserTaskModel {
                user_id:    user.id().into(),
                task_year:  ut.task_id().year,
                task_week:  ut.task_id().week as i32,
                partner_id: ut.partner_id().into(),
                state:      ut.state().clone().into(),
            }),
        )
    }
}

impl Into<UserTaskState> for UserTaskStateModel {
    fn into(self) -> UserTaskState {
        match self {
            UserTaskStateModel::Active => UserTaskState::Active,
            UserTaskStateModel::Completed => UserTaskState::Completed,
        }
    }
}

impl Into<NextTaskStatus> for NextTaskStatusModel {
    fn into(self) -> NextTaskStatus {
        match self {
            NextTaskStatusModel::Pending => NextTaskStatus::Pending,
            NextTaskStatusModel::Accepted => NextTaskStatus::Accepted,
            NextTaskStatusModel::Rejected => NextTaskStatus::Rejected,
        }
    }
}

impl Into<UserTask> for UserTaskModel {
    fn into(self) -> UserTask {
        UserTask::restore(
            TaskId::new(self.task_year, self.task_week as u32),
            self.partner_id,
            self.state.into(),
        )
    }
}

impl Into<User> for (UserModel, Option<UserTaskModel>) {
    fn into(self) -> User {
        let (user_model, user_task_model) = self;

        let profile = user_model.full_name.map(|full_name| {
            Profile::new(
                full_name,
                user_model.group_name.expect("group name must exist if full name exists"),
            )
        });

        let user_task = user_task_model.map(|ut| ut.into());

        User::restore(
            user_model.id,
            user_model.username,
            profile,
            user_task,
            user_model.next_task_status.into(),
            user_model.completed_tasks,
        )
    }
}

impl From<Row> for UserModel {
    fn from(row: Row) -> Self {
        Self {
            id:                 row.get("id"),
            username:           row.get("username"),
            full_name:          row.get("full_name"),
            group_name:         row.get("group_name"),
            next_task_status:   row.get("next_task_status"),
            completed_tasks:    row.get("completed_tasks"),
        }
    }
}

impl From<Row> for UserTaskModel {
    fn from(row: Row) -> Self {
        Self {
            user_id:    row.get("user_id"),
            task_year:  row.get("task_year"),
            task_week:  row.get("task_week"),
            partner_id: row.get("partner_id"),
            state:      row.get("state"),
        }
    }
}

#[cfg(test)]
mod integration_tests {
    use deadpool_postgres::GenericClient;
    use futures::future::join_all;
    use rand::random;
    use crate::utils::postgres::testing::test_db_setup;
    use super::*;

    fn create_user_from_id(id: i64) -> User {
        User::new(id, format!("user{}", id))
    }

    #[tokio::test]
    async fn test_save_and_retrieve_user() {
        let pool = test_db_setup().await;
        let repo = PostgresUserRepository::new(pool.clone());
        let test_user = create_user_from_id(random());

        repo.save(&test_user)
            .await
            .expect("failed to save user");

        let retrieved_user = repo.user(test_user.id())
            .await
            .expect("failed to retrieve user");

        assert_eq!(test_user, retrieved_user);
    }

    #[tokio::test]
    async fn test_save_user_with_task() {
        let pool = test_db_setup().await;
        let repo = PostgresUserRepository::new(pool.clone());
        let mut test_user = create_user_from_id(random());

        let task_id = TaskId::new(2025, 10);
        {
            let client = pool
                .get()
                .await
                .expect("failed to get client from pool");

            client.execute(
                r#"
                INSERT INTO tasks (
                    year,
                    week,
                    title,
                    description
                ) VALUES ($1, $2, 'Test task', 'Lorem ispum')
                "#,
                &[
                    &task_id.year,
                    &(task_id.week as i32),
                ],
            )
                .await
                .expect("failed to insert test task");
        }

        test_user.accept().expect("failed to accept next task");
        test_user.promote(test_user.id(), task_id).expect("failed to promote task");

        repo.save(&test_user)
            .await
            .expect("failed to save user");

        let retrieved_user = repo.user(test_user.id())
            .await
            .expect("failed to retrieve user");

        assert_eq!(test_user, retrieved_user);
    }

    #[tokio::test]
    async fn test_user_exists() {
        let pool = test_db_setup().await;
        let repo = PostgresUserRepository::new(pool.clone());
        let test_user = create_user_from_id(random());

        repo.save(&test_user)
            .await
            .expect("failed to save user");

        let result = repo.save(&test_user)
            .await;

        match result {
            Err(DomainError::UserAlreadyExists(id)) => {
                assert_eq!(id, test_user.id(), "returned user ID should match the requested one");
            },
            Ok(_) => panic!("expected UserAlreadyExists error, but got Ok"),
            Err(e) => panic!("expected UserAlreadyExists error, but got different error: {:?}", e),
        }
    }

    #[tokio::test]
    async fn test_update_user() {
        let pool = test_db_setup().await;
        let repo = PostgresUserRepository::new(pool.clone());
        let mut test_user = create_user_from_id(random());

        repo.save(&test_user)
            .await
            .expect("failed to save user");

        test_user.set_profile(Profile::new("Ivanov Ivan", "СМ13-13Б"));
        repo.update(&test_user)
            .await
            .expect("failed to update user");

        let updated_user = repo.user(test_user.id())
            .await
            .expect("failed to retrieve updated user");

        assert_eq!(test_user, updated_user);
    }

    #[tokio::test]
    async fn test_find_user_exists() {
        let pool = test_db_setup().await;
        let repo = PostgresUserRepository::new(pool.clone());
        let test_user = create_user_from_id(random());

        repo.save(&test_user)
            .await
            .expect("failed to save user");

        let found_user = repo.find_user(test_user.id())
            .await
            .expect("failed to find user");

        assert!(found_user.is_some());
        assert_eq!(test_user, found_user.unwrap());
    }

    #[tokio::test]
    async fn test_find_user_not_exists() {
        let pool = test_db_setup().await;
        let repo = PostgresUserRepository::new(pool.clone());
        let non_existent_id = UserId(random());

        let found_user = repo.find_user(non_existent_id)
            .await
            .expect("failed to execute find_user");

        assert!(found_user.is_none());
    }

    #[tokio::test]
    async fn test_get_all_users() {
        let pool = test_db_setup().await;
        let repo = PostgresUserRepository::new(pool.clone());

        let users: Vec<User> = join_all((0..3).map(async |_| {
            let user = create_user_from_id(random());
            repo.save(&user)
                .await
                .expect("Failed to save user");
            user
        })).await;

        let all_users = repo.all()
            .await
            .expect("Failed to get all users");

        assert!(all_users.len() >= 3);
        assert!(all_users.iter().any(|u| u.id() == users[0].id()));
        assert!(all_users.iter().any(|u| u.id() == users[1].id()));
        assert!(all_users.iter().any(|u| u.id() == users[2].id()));
    }

    #[tokio::test]
    async fn test_get_free_users() {
        let pool = test_db_setup().await;
        let repo = PostgresUserRepository::new(pool.clone());

        let pending_user = create_user_from_id(random());
        let accepted_user = {
            let mut user = create_user_from_id(random());
            user.accept()
                .expect("failed to accept next task by user");
            user
        };

        repo.save(&pending_user).await.expect("failed to save user");
        repo.save(&accepted_user).await.expect("failed to save user");

        let accepted_users = repo.ready_users()
            .await
            .expect("failed to get free users");

        assert!(accepted_users.len() >= 1);
        assert!(accepted_users.iter().any(|u| u.id() == accepted_user.id()));
        assert!(accepted_users.iter().all(|u| u.id() != pending_user.id()));
    }
}
