use std::sync::Arc;
use chrono::{Duration, Utc};
use crate::core::bot::CwBot;
use crate::domain::interfaces::{QuestRepository, UserRepository};
use crate::domain::models::{Profile, User};
use crate::domain::use_cases::{AcceptMeetingUseCase, CheckAdminUseCase, CompleteRegistrationUseCase,
                               GetAllUsersUseCase, GetCurrentMeetingUseCase, GetMenuStateUseCase, 
                               GetNextMeetingUseCase, RejectMeetingUseCase, StartRegistrationUseCase
};
use crate::services::{InMemoryQuestRepository, InMemoryUserRepository, MockAuthService};

mod core;
mod domain;
mod services;
mod presentation;

fn user_from_num(i: i32) -> User {
    let mut user = User::new(
        i as i64,
        format!("user_{i}")
    );
    user.set_profile(Profile::new(
        format!("User {i}"),
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
    let quest_repo = Arc::new(InMemoryQuestRepository::default());
    let auth_service = Arc::new(MockAuthService::with_admin_ids(&[1723307580]));

    quest_repo.create(
        "1 Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum",
    ).await
        .expect("failed to save quest");

    quest_repo.create(
        "2 Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum",
    ).await
        .expect("failed to save quest");

    /*quest_repo.create(
        "2 Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum",
        Utc::now() + Duration::days(7), Utc::now() + Duration::days(14),
    ).await
        .expect("Failed to save quest");*/

    for i in 1..=30 {
        user_repo.save(&user_from_num(i)).await.expect("failed to save user");
    }

    let start_registration_use_case = StartRegistrationUseCase::new(user_repo.clone());
    let complete_registration_use_case = CompleteRegistrationUseCase::new(user_repo.clone());
    let accept_meeting_use_case = AcceptMeetingUseCase::new(user_repo.clone());
    let reject_meeting_use_case = RejectMeetingUseCase::new(user_repo.clone());
    let get_next_meeting_use_case = GetNextMeetingUseCase::new(user_repo.clone(), quest_repo.clone());
    let get_menu_state_use_case = GetMenuStateUseCase::new(user_repo.clone());
    let get_current_meeting_use_case = GetCurrentMeetingUseCase::new(
        user_repo.clone(),
        quest_repo.clone(),
    );
    let check_admin_use_case = CheckAdminUseCase::new(auth_service.clone());
    let get_all_users_use_case = GetAllUsersUseCase::new(user_repo.clone());

    CwBot::new().run(
        start_registration_use_case,
        complete_registration_use_case,
        accept_meeting_use_case,
        reject_meeting_use_case,
        get_next_meeting_use_case,
        get_menu_state_use_case,
        get_current_meeting_use_case,
        check_admin_use_case,
        get_all_users_use_case,
    ).await
}
