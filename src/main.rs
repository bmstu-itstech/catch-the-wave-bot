use std::env;
use std::sync::Arc;
use teloxide::prelude::*;

use crate::dispatcher::CwDispatcher;
use crate::domain::use_cases::*;
use crate::services::*;
use crate::utils::postgres::pool;

mod domain;
mod services;
mod presentation;
mod dispatcher;
mod utils;


#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    
    let uri = env::var("DATABASE_URI")
        .expect("DATABASE_URI must be set");
    let pool = pool::connect(&uri)
        .expect(format!("unable to connect to database: {}", uri).as_str());
    log::info!("Connected to PostgreSQL database: {}", uri);

    let admin_ids_str = env::var("ADMIN_IDS")
        .expect("ADMIN_IDS must be set");

    let admin_ids: Vec<i64> = admin_ids_str
        .split(',')
        .map(|s| s.trim().parse().expect("invalid admin ID format"))
        .collect();

    let user_repo = Arc::new(PostgresUserRepository::new(pool.clone()));
    let task_repo = Arc::new(PostgresTaskRepository::new(pool.clone()));
    let auth_service = Arc::new(MockAuthService::with_admin_ids(admin_ids));
    let week_service = Arc::new(ChronoWeekService::default());

    let start_registration_use_case = StartRegistrationUseCase::new(user_repo.clone());
    let complete_registration_use_case = CompleteRegistrationUseCase::new(user_repo.clone());
    let accept_next_task_use_case = AcceptNextTaskUseCase::new(user_repo.clone());
    let reject_next_task_use_case = RejectTaskUseCase::new(user_repo.clone());
    let get_menu_state_use_case = GetMenuStateUseCase::new(user_repo.clone());
    let get_current_meeting_use_case = GetUserTaskUseCase::new(user_repo.clone(), task_repo.clone());
    let check_admin_use_case = CheckAdminUseCase::new(auth_service.clone());
    let get_all_users_use_case = GetAllUsersUseCase::new(user_repo.clone());
    let get_user_use_case = GetUserUseCase::new(user_repo.clone(), task_repo.clone());
    let get_free_users_use_case = GetFreeUsersUseCase::new(user_repo.clone());
    let assign_partner_use_case = AssignPartnerUseCase::new(user_repo.clone(), task_repo.clone(), week_service.clone());
    let check_next_task_use_case = CheckNextTaskUseCase::new(task_repo.clone(), week_service.clone());
    let create_next_task_use_case = CreateNextTaskUseCase::new(task_repo.clone(), week_service.clone());
    
    log::info!("Starting bot...");
    
    let bot = Bot::from_env();
    let mut dispatcher = CwDispatcher::create(
        bot,
        start_registration_use_case,
        complete_registration_use_case,
        accept_next_task_use_case,
        reject_next_task_use_case,
        get_menu_state_use_case,
        get_current_meeting_use_case,
        check_admin_use_case,
        get_all_users_use_case,
        get_user_use_case,
        get_free_users_use_case,
        assign_partner_use_case,
        check_next_task_use_case,
        create_next_task_use_case,
    ).await;
    dispatcher.dispatch().await;
}
