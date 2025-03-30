use teloxide::prelude::Bot;

use crate::core::dispatcher::CwDispatcher;
use crate::domain::use_cases::registration::RegistrationUseCase;

pub struct CwBot {
    bot: Bot,
    reg_case: RegistrationUseCase,
}

impl CwBot {
    pub fn new(
        reg_case: RegistrationUseCase,
    ) -> Self {
        Self {
            bot: Bot::from_env(),
            reg_case,
        }
    }

    pub async fn run(&self) {
        log::info!("Starting bot...");
        
        let mut dispatcher = CwDispatcher::create(self.bot.clone(), self.reg_case.clone()).await;
        dispatcher.dispatch().await;
    }
}
