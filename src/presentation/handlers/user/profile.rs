use teloxide::prelude::*;
use teloxide::types::{InlineKeyboardButton, InlineKeyboardMarkup, ParseMode};

use crate::domain::use_cases::GetUserUseCase;
use crate::presentation::handlers::fsm::CwDialogueState;
use crate::presentation::handlers::texts::T;
use crate::presentation::handlers::utils::{CwBotError, CwDialogue, CwHandlerResult};


pub async fn handle_profile_callback(
    bot: Bot,
    q: CallbackQuery,
    dialogue: CwDialogue,
    use_case: GetUserUseCase,
) -> CwHandlerResult {
    bot.answer_callback_query(&q.id).await?;
    
    let user = use_case.execute(dialogue.chat_id().0)
        .await
        .map_err(|err| CwBotError::Other(err.to_string()))?;
    
    let text = T.profile.profile(
        &user.full_name,
        &user.group_name,
        &user.next_task_status,
        &user.completed_quests,
    );
    
    bot.send_message(dialogue.chat_id(), text)
        .parse_mode(ParseMode::Html)
        .reply_markup(InlineKeyboardMarkup::new(vec![
            vec![
                InlineKeyboardButton::callback(
                    T.profile.re_register_button, "menu_profile_re_register",
                )
            ]
        ]
        ))
        .await?;
    Ok(())
}


pub async fn handle_re_register_callback(
    bot: Bot,
    q: CallbackQuery,
    dialogue: CwDialogue,
) -> CwHandlerResult {
    bot.answer_callback_query(&q.id).await?;
    bot.send_message(dialogue.chat_id(), T.registration.enter_full_name).await?;
    dialogue.update(CwDialogueState::AwaitingFullName).await?;
    Ok(())
}
