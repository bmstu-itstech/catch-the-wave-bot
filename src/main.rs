use std::sync::Arc;
use teloxide::prelude::*;

use crate::dispatcher::CwDispatcher;
use crate::domain::interfaces::UserRepository;
use crate::domain::models::{Profile, User};

use crate::domain::use_cases::*;
use crate::services::*;

mod domain;
mod services;
mod presentation;
mod dispatcher;


fn user_from_num(i: i32) -> User {
    let mut user = User::new(
        i as i64,
        format!("user_{i}")
    );
    user.set_profile(Profile::new(
        format!("User{i}"),
        format!("TT-1{i}"),
    ));
    user
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    pretty_env_logger::init();
    log::info!("Starting bot...");

    let user_repo = Arc::new(InMemoryUserRepository::default());
    let task_repo = Arc::new(InMemoryTaskRepository::default());
    let auth_service = Arc::new(MockAuthService::with_admin_ids(vec![1723307580]));
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
    
    let mut user1 = User::new(1, "testuser");
    user1.set_profile(Profile::new("Иванов Иван Иванович", "СМ13-13Б"));
    user1.accept()
        .expect("failed to accept next task");
    user_repo.save(&user1).await
        .expect("failed to save user");
    
    for i in 2..30 {
        let user = user_from_num(i);
        user_repo.save(&user).await
            .expect("failed to save user");
    }
    
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
