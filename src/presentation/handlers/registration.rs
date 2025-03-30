use teloxide::prelude::*;

use crate::core::fsm::CwDialogueState;
use crate::domain::use_cases::registration::RegistrationUseCase;

use super::texts::T;
use super::utils::{CwDialogue, CwHandlerResult};


pub async fn start_registration(
    bot: Bot,
    msg: Message,
    dialogue: CwDialogue,
    reg_case: RegistrationUseCase,
) -> CwHandlerResult {
    let username = msg.chat.username().map(|u| u.to_string());

    reg_case.start_registration(msg.chat.id.0, username).await?;
    log::info!("user @{:?} has started registration", msg.chat.username());

    dialogue.update(CwDialogueState::AwaitingFullName).await?;

    bot.send_message(msg.chat.id, T.registration.start).await?;
    bot.send_message(msg.chat.id, T.registration.enter_full_name).await?;

    Ok(())
}

pub async fn receive_full_name(
    bot: Bot,
    msg: Message,
    dialogue: CwDialogue,
) -> CwHandlerResult {
    let full_name = match msg.text() {
        Some(text) => text.to_string(),
        None => return Ok(()),
    };
    log::info!("received full name for @{:?}: {}", msg.chat.username(), full_name);

    dialogue.update(CwDialogueState::AwaitingGroupName { full_name }).await?;
    
    bot.send_message(msg.chat.id, T.registration.enter_group_name).await?;

    Ok(())
}

pub async fn receive_group_name(
    bot: Bot,
    msg: Message,
    full_name: String,
    dialogue: CwDialogue,
    text: String,
    reg_case: RegistrationUseCase,
) -> CwHandlerResult {
    let group_name = text;
    log::info!("received group name for @{:?}: {}", msg.chat.username(), group_name);

    let user = reg_case.complete_registration(
        msg.chat.id.0,
        full_name,
        group_name,
    ).await?;
    log::info!("user @{:?} has completed registration: {:?}", msg.chat.username(), user);

    dialogue.update(CwDialogueState::Idle).await?;

    bot.send_message(msg.chat.id, T.registration.registration_complete).await?;

    Ok(())
}
