use teloxide::prelude::Bot;

use crate::core::dispatcher::CwDispatcher;
use crate::domain::use_cases::{AcceptMeetingUseCase, CheckAdminUseCase, CompleteRegistrationUseCase,
                               GetAllUsersUseCase, GetCurrentMeetingUseCase, GetMenuStateUseCase,
                               GetNextMeetingUseCase, RejectMeetingUseCase, StartRegistrationUseCase
};

pub struct CwBot {
    bot: Bot,
}

impl CwBot {
    pub fn new() -> Self {
        Self {
            bot: Bot::from_env(),
        }
    }

    pub async fn run(
        &self,
        start_registration_use_case: StartRegistrationUseCase,
        complete_registration_use_case: CompleteRegistrationUseCase,
        accept_meeting_use_case: AcceptMeetingUseCase,
        reject_meeting_use_case: RejectMeetingUseCase,
        get_next_meeting_use_case: GetNextMeetingUseCase,
        get_menu_state_use_case: GetMenuStateUseCase,
        get_current_meeting_use_case: GetCurrentMeetingUseCase,
        check_admin_use_case: CheckAdminUseCase,
        get_all_users_use_case: GetAllUsersUseCase,
    ) {
        log::info!("Starting bot...");
        
        let mut dispatcher = CwDispatcher::create(
            self.bot.clone(), 
            start_registration_use_case,
            complete_registration_use_case,
            accept_meeting_use_case,
            reject_meeting_use_case,
            get_next_meeting_use_case,
            get_menu_state_use_case,
            get_current_meeting_use_case,
            check_admin_use_case,
            get_all_users_use_case,
        ).await;
        dispatcher.dispatch().await;
    }
}
