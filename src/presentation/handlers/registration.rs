use teloxide::prelude::*;

use crate::core::fsm::CwDialogueState;
use crate::domain::error::DomainError;
use crate::domain::use_cases::{
    CompleteRegistrationUseCase, GetMenuStateUseCase, StartRegistrationUseCase,
};
use crate::presentation::handlers::menu::send_menu;
use super::texts::T;
use super::utils::{CwBotError, CwDialogue, CwHandlerResult};


pub async fn handle_start_command(
    bot: Bot,
    msg: Message,
    dialogue: CwDialogue,
    start_registration_use_case: StartRegistrationUseCase,
    get_menu_state_use_case: GetMenuStateUseCase,
) -> CwHandlerResult {
    let username = msg.chat.username()
        .ok_or(CwBotError::Other(format!("no username for user id {}", msg.chat.id.0)))?;
    
    if let Err(err) = start_registration_use_case.execute(msg.chat.id.0, username).await {
        return match err {
            DomainError::UserAlreadyExists(_) =>
                send_menu(bot, msg, get_menu_state_use_case).await,
            _ => Err(CwBotError::Other(err.to_string())),
        }
    }

    log::info!("user @{:?} has started registration", msg.chat.username());

    dialogue.update(CwDialogueState::AwaitingFullName).await?;

    bot.send_message(msg.chat.id, T.registration.start).await?;
    bot.send_message(msg.chat.id, T.registration.enter_full_name).await?;

    Ok(())
}

pub async fn handle_re_register_command(
    bot: Bot,
    msg: Message,
    dialogue: CwDialogue,
    use_case: StartRegistrationUseCase,
) -> CwHandlerResult {
    let username = msg.chat.username()
        .ok_or(CwBotError::Other(format!("no username for user id {}", msg.chat.id.0)))?;
    
    use_case.execute(msg.chat.id.0, username).await
        .map_err(|err| CwBotError::Other(err.to_string()))?;

    log::info!("user @{:?} has started re-registration", msg.chat.username());

    dialogue.update(CwDialogueState::AwaitingFullName).await?;

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
    complete_registration_use_case: CompleteRegistrationUseCase,
    get_menu_state_use_case: GetMenuStateUseCase,
) -> CwHandlerResult {
    let group_name = text;
    log::info!("received group name for @{:?}: {}", msg.chat.username(), group_name);

    let user = match complete_registration_use_case.execute(
        msg.chat.id.0,
        full_name.as_str(),
        group_name.as_str(),
    ).await {
        Ok(user) => user,
        Err(err) => return Err(CwBotError::Other(err.to_string())),
    };
    
    log::info!("user @{:?} has completed registration: {:?}", msg.chat.username(), user);

    dialogue.update(CwDialogueState::Idle).await?;

    bot.send_message(msg.chat.id, T.registration.registration_complete).await?;
    send_menu(bot, msg, get_menu_state_use_case).await
}
