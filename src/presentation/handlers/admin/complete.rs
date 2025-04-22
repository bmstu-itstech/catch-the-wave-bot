use teloxide::prelude::*;

use crate::domain::use_cases::{CompleteTaskUseCase, GetActiveUsersUseCase};
use crate::presentation::handlers::admin::{build_admin_menu_users_keyboard, delete_callback_message};
use crate::presentation::handlers::fsm::CwDialogueState;
use crate::presentation::handlers::texts::T;
use crate::presentation::handlers::utils::{CwBotError, CwDialogue, CwHandlerResult};


pub async fn handle_admin_menu_complete_callback(
    bot: Bot,
    q: CallbackQuery,
    dialogue: CwDialogue,
    get_active_users_use_case: GetActiveUsersUseCase,
) -> CwHandlerResult {
    bot.answer_callback_query(&q.id).await?;

    let users = get_active_users_use_case.execute().await
        .map_err(|err| CwBotError::External(err.into()))?;

    if users.is_empty() {
        bot.send_message(dialogue.chat_id(), T.admin_complete_task.no_users)
            .await?;
        return Ok(());
    }

    let keyboard = build_admin_menu_users_keyboard(&users);
    bot.send_message(dialogue.chat_id(), T.admin_complete_task.text)
        .reply_markup(keyboard)
        .await?;
    dialogue.update(CwDialogueState::AwaitingUserForComplete).await?;

    Ok(())
}

pub async fn handle_admin_menu_complete_user_callback(
    bot: Bot,
    q: CallbackQuery,
    dialogue: CwDialogue,
    use_case: CompleteTaskUseCase,
) -> CwHandlerResult {
    bot.answer_callback_query(&q.id).await?;
    delete_callback_message(&bot, &q).await?;

    let user_id: i64 = q.data
        .as_ref()
        .unwrap()
        .split(":")
        .last()
        .unwrap()
        .parse()
        .unwrap();

    use_case.execute(user_id)
        .await
        .map_err(|err| CwBotError::External(err.into()))?;

    bot.send_message(dialogue.chat_id(), T.admin_complete_task.success)
        .await?;

    Ok(())
}


