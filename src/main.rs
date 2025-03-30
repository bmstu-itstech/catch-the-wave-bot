use std::sync::Arc;

use crate::core::bot::CwBot;
use crate::domain::use_cases::registration::RegistrationUseCase;
use crate::services::InMemoryUserRepository;

mod core;
mod domain;
mod services;
mod presentation;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    pretty_env_logger::init();
    log::info!("Starting bot...");

    let user_repo = InMemoryUserRepository::default();
    let reg_case = RegistrationUseCase::new(Arc::new(user_repo));

    CwBot::new(reg_case)
        .run().await
}
