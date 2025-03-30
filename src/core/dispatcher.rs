use teloxide::{filter_command, Bot};
use teloxide::dispatching::dialogue::InMemStorage;
use teloxide::dispatching::{dialogue, DefaultKey, Dispatcher, UpdateHandler};
use teloxide::dptree::case;
use teloxide::prelude::*;

use crate::core::fsm::CwDialogueState;
use crate::domain::use_cases::registration::RegistrationUseCase;
use crate::presentation::handlers::commands::Command;
use crate::presentation::handlers::registration;


pub struct CwDispatcher;

impl CwDispatcher {
    pub async fn create(
        bot: Bot,
        reg_case: RegistrationUseCase,
    ) -> Dispatcher<Bot, Box<dyn std::error::Error + Send + Sync>, DefaultKey> {
        Dispatcher::builder(bot, Self::schema())
            .dependencies(dptree::deps![
                InMemStorage::<CwDialogueState>::new(),
                reg_case
            ])
            .default_handler(|upd| async move {
                log::warn!("Unhandled update: {:?}", upd);
            })
            .enable_ctrlc_handler()
            .build()
    }

    fn schema() -> UpdateHandler<Box<dyn std::error::Error + Send + Sync + 'static>> {
        let command_handler = filter_command::<Command, _>()
            .branch(case![Command::Start].endpoint(registration::start_registration));

        let message_handler = Update::filter_message()
            .enter_dialogue::<Message, InMemStorage<CwDialogueState>, CwDialogueState>()
            .branch(
                case![CwDialogueState::AwaitingFullName]
                    .endpoint(registration::receive_full_name)
            )
            .branch(
                case![CwDialogueState::AwaitingGroupName { full_name }]
                    .endpoint(registration::receive_group_name)
            );

        let compose_handler = Update::filter_message()
            .branch(command_handler)
            .branch(message_handler);
        
        dialogue::enter::<Update, InMemStorage<CwDialogueState>, CwDialogueState, _>()
            .branch(compose_handler)
    }
}
