use teloxide::{filter_command, Bot};
use teloxide::dispatching::dialogue::InMemStorage;
use teloxide::dispatching::{dialogue, DefaultKey, Dispatcher, UpdateHandler};
use teloxide::prelude::*;

use crate::core::fsm::CwDialogueState;
use crate::domain::use_cases::{AcceptMeetingUseCase, CompleteRegistrationUseCase, StartRegistrationUseCase, RejectMeetingUseCase, GetNextMeetingUseCase, GetMenuStateUseCase, MenuCategory, GetCurrentMeetingUseCase};
use crate::presentation::handlers::commands::Command;
use crate::presentation::handlers::{current_meeting, next_meeting, registration};
use crate::presentation::handlers::menu::MenuCallback;
use crate::presentation::handlers::next_meeting::NextMeeting;
use crate::presentation::handlers::utils::CwBotError;

pub struct CwDispatcher;

impl CwDispatcher {
    pub async fn create(
        bot: Bot,
        start_registration_use_case: StartRegistrationUseCase,
        complete_registration_use_case: CompleteRegistrationUseCase,
        accept_meeting_use_case: AcceptMeetingUseCase,
        reject_meeting_use_case: RejectMeetingUseCase,
        get_next_meeting_use_case: GetNextMeetingUseCase,
        get_menu_state_use_case: GetMenuStateUseCase,
        get_current_meeting_use_case: GetCurrentMeetingUseCase
    ) -> Dispatcher<Bot, CwBotError, DefaultKey> {
        Dispatcher::builder(bot, Self::schema())
            .dependencies(dptree::deps![
                InMemStorage::<CwDialogueState>::new(),
                start_registration_use_case,
                complete_registration_use_case,
                accept_meeting_use_case,
                reject_meeting_use_case,
                get_menu_state_use_case,
                get_next_meeting_use_case,
                get_current_meeting_use_case
            ])
            .default_handler(|upd| async move {
                log::warn!("Unhandled update: {:?}", upd);
            })
            .enable_ctrlc_handler()
            .build()
    }

    fn schema() -> UpdateHandler<CwBotError> {
        use dptree::case;
        
        let command_handler = filter_command::<Command, _>()
            //.branch(case![Command::Cancel].endpoint(registration::handle_cancel))
            .branch(case![Command::Start].endpoint(registration::handle_start_command))
            //.branch(case![Command::NextMeeting].endpoint(next_meeting::handle_next_meeting_command))
            //.branch(case![Command::ReRegister].endpoint(registration::handle_re_register_command));
        ;

        let message_handler = Update::filter_message()
            .branch(
                case![CwDialogueState::AwaitingFullName]
                    .endpoint(registration::receive_full_name)
            )
            .branch(
                case![CwDialogueState::AwaitingGroupName { full_name }]
                    .endpoint(registration::receive_group_name)
            )
            .branch(
                case![CwDialogueState::AwaitingAcceptNextMeeting]
                    .filter(|msg: Message| msg.text().map(String::from) == Some(NextMeeting::Accept.into()))
                    .endpoint(next_meeting::handle_next_meeting_accept)
            )
            .branch(
                case![CwDialogueState::AwaitingAcceptNextMeeting]
                    .filter(|msg: Message| msg.text().map(String::from) == Some(NextMeeting::Reject.into()))
                    .endpoint(next_meeting::handle_next_meeting_reject)
            )
        ;

        let callback_handler = Update::filter_callback_query()
            .filter_map(extract_action)
            .branch(
                case![MenuCallback::NextMeeting]
                    .endpoint(next_meeting::handle_next_meeting_callback)
            )
            .branch(
                case![MenuCallback::CurrentMeeting]
                    .endpoint(current_meeting::handle_current_meeting_callback)
            )
        ;
        
        let compose_handler = Update::filter_message()
            .branch(command_handler)
            .branch(message_handler)
        ;
        
        dialogue::enter::<Update, InMemStorage<CwDialogueState>, CwDialogueState, _>()
            .branch(callback_handler)
            .branch(compose_handler)
    }
}

fn extract_action(q: CallbackQuery) -> Option<MenuCallback> {
    q.data.and_then(|str| MenuCallback::try_from(str).ok())
}
