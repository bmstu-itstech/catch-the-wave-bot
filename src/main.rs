use std::sync::Arc;
use chrono::Utc;
use crate::core::bot::CwBot;
use crate::domain::interfaces::{QuestRepository, UserRepository};
use crate::domain::models::{Quest, User};
use crate::domain::use_cases::{
    AcceptMeetingUseCase, CompleteRegistrationUseCase, GetCurrentMeetingUseCase,
    GetMenuStateUseCase, GetNextMeetingUseCase, RejectMeetingUseCase, StartRegistrationUseCase,
};
use crate::services::{InMemoryQuestRepository, InMemoryUserRepository};

mod core;
mod domain;
mod services;
mod presentation;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    pretty_env_logger::init();
    log::info!("Starting bot...");

    let user_repo = Arc::new(InMemoryUserRepository::default());
    let quest_repo = Arc::new(InMemoryQuestRepository::default());

    user_repo.save(User::new(1, String::from("some_partner"))).await
        .expect("Failed to save user");
    quest_repo.save(Quest::new(
        1, Utc::now(), Utc::now(),
        String::from("Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum")
    )).await
        .expect("Failed to save quest");
    
    let start_registration_use_case = StartRegistrationUseCase::new(user_repo.clone());
    let complete_registration_use_case = CompleteRegistrationUseCase::new(user_repo.clone());
    let accept_meeting_use_case = AcceptMeetingUseCase::new(user_repo.clone());
    let reject_meeting_use_case = RejectMeetingUseCase::new(user_repo.clone());
    let get_next_meeting_use_case = GetNextMeetingUseCase::new(user_repo.clone());
    let get_menu_state_use_case = GetMenuStateUseCase::new(user_repo.clone());
    let get_current_meeting_use_case = GetCurrentMeetingUseCase::new(
        user_repo.clone(),
        quest_repo.clone(),
    );

    CwBot::new().run(
        start_registration_use_case,
        complete_registration_use_case,
        accept_meeting_use_case,
        reject_meeting_use_case,
        get_next_meeting_use_case,
        get_menu_state_use_case,
        get_current_meeting_use_case,
    ).await
}
