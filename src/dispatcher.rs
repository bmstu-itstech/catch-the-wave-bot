use teloxide::{filter_command, Bot};
use teloxide::dispatching::dialogue::InMemStorage;
use teloxide::dispatching::{dialogue, DefaultKey, Dispatcher, UpdateHandler};
use teloxide::prelude::*;

use crate::domain::use_cases::*;

use crate::presentation::handlers::commands::Command;
use crate::presentation::handlers::{admin, user};
use crate::presentation::handlers::fsm::CwDialogueState;
use crate::presentation::handlers::utils::CwBotError;


pub struct CwDispatcher;

impl CwDispatcher {
    pub async fn create(
        bot: Bot,
        start_registration_use_case: StartRegistrationUseCase,
        complete_registration_use_case: CompleteRegistrationUseCase,
        accept_next_task_use_case: AcceptNextTaskUseCase,
        reject_next_task_use_case: RejectTaskUseCase,
        get_menu_state_use_case: GetMenuStateUseCase,
        get_current_meeting_use_case: GetUserTaskUseCase,
        check_admin_use_case: CheckAdminUseCase,
        get_all_users_use_case: GetAllUsersUseCase,
        get_user_use_case: GetUserUseCase,
        get_free_users_use_case: GetFreeUsersUseCase,
        assign_partner_use_case: AssignPartnerUseCase,
    ) -> Dispatcher<Bot, CwBotError, DefaultKey> {
        Dispatcher::builder(bot, Self::schema())
            .dependencies(dptree::deps![
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
                InMemStorage::<CwDialogueState>::new()
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
            .branch(case![Command::Start].endpoint(user::handle_start_command))
            .branch(case![Command::Admin].endpoint(admin::handle_admin_command))
        ;

        let message_handler = Update::filter_message()
            .branch(
                case![CwDialogueState::AwaitingFullName]
                    .endpoint(user::receive_full_name)
            )
            .branch(
                case![CwDialogueState::AwaitingGroupName { full_name }]
                    .endpoint(user::receive_group_name)
            )
            .branch(
                case![CwDialogueState::AwaitingAcceptNextMeeting]
                    .filter(|msg: Message| msg.text().map(String::from) == Some(user::NextTaskCallback::Accept.into()))
                    .endpoint(user::handle_next_meeting_accept)
            )
            .branch(
                case![CwDialogueState::AwaitingAcceptNextMeeting]
                    .filter(|msg: Message| msg.text().map(String::from) == Some(user::NextTaskCallback::Reject.into()))
                    .endpoint(user::handle_next_meeting_reject)
            )
        ;

        let callback_handler = Update::filter_callback_query()
            .branch(
                dptree::entry()
                    .filter_map(extract_menu_callback)
                    .branch(
                        case![user::MenuCallback::NextTask]
                            .endpoint(user::handle_next_meeting_callback)
                    )
                    .branch(
                        case![user::MenuCallback::UserTask]
                            .endpoint(user::handle_current_meeting_callback)
                    )
                    .branch(
                        case![user::MenuCallback::Rules]
                            .endpoint(user::handle_rules_callback)
                    )
                    .branch(
                        case![user::MenuCallback::Profile]
                            .endpoint(user::handle_profile_callback)
                    )
            )
            .branch(
                dptree::entry()
                    .filter_map(extract_admin_menu_callback)
                    .branch(
                        case![admin::AdminMenuCallback::Users]
                            .endpoint(admin::handle_admin_menu_users_callback)
                    )
                    .branch(
                        case![admin::AdminMenuCallback::AssignPartner]
                            .endpoint(admin::handle_admin_menu_assign_partner_callback)
                    )
                    /*.branch(
                        case![AdminMenuCallback::Meetings]
                            .endpoint(admin::handle_admin_menu_meetings_callback)
                    )*/
            )
            .branch(
                dptree::entry()
                    .filter_map(is_admin_menu_user_callback)
                    .branch(
                        case![true]
                            .branch(
                                case![CwDialogueState::AwaitingUser]
                                    .endpoint(admin::handle_admin_menu_user_callback)
                            )
                            .branch(
                                case![CwDialogueState::AwaitingPartner1]
                                    .endpoint(admin::handle_admin_menu_assign_partner_1_callback)
                            )
                            .branch(
                                case![CwDialogueState::AwaitingPartner2 { partner_1_id } ]
                                    .endpoint(admin::handle_admin_menu_assign_partner_2_callback)
                            )
                    )
            )
            .branch(
                dptree::entry()
                    .filter_map(is_profile_re_register_callback)
                    .branch(
                        case![true]
                            .endpoint(user::handle_re_register_callback)
                    )
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

fn extract_menu_callback(q: CallbackQuery) -> Option<user::MenuCallback> {
    q.data.and_then(|str| user::MenuCallback::try_from(str).ok())
}

fn extract_admin_menu_callback(q: CallbackQuery) -> Option<admin::AdminMenuCallback> {
    q.data.and_then(|str| admin::AdminMenuCallback::try_from(str).ok())
}

fn is_admin_menu_user_callback(q: CallbackQuery) -> Option<bool> {
    Some(q.data?.starts_with("admin_menu_user:"))
}

fn is_profile_re_register_callback(q: CallbackQuery) -> Option<bool> {
    Some(q.data?.eq("menu_profile_re_register"))
}
