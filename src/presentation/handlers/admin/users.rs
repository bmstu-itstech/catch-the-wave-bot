use teloxide::prelude::*;
use teloxide::types::{InlineKeyboardButton, InlineKeyboardMarkup};

use crate::domain::use_cases::{GetAllUsersUseCase, UserDto};
use crate::presentation::handlers::fsm::CwDialogueState;
use crate::presentation::handlers::texts::T;
use crate::presentation::handlers::utils::{CwBotError, CwDialogue, CwHandlerResult};


pub async fn handle_admin_menu_users_callback(
    bot: Bot,
    q: CallbackQuery,
    dialogue: CwDialogue,
    use_case: GetAllUsersUseCase,
) -> CwHandlerResult {
    bot.answer_callback_query(&q.id).await?;

    let users = use_case.execute().await
        .map_err(|err| CwBotError::External(err.into()))?;

    if users.is_empty() {
        bot.send_message(dialogue.chat_id(), T.admin_users.no_users)
            .await?;
        return Ok(());
    }
    
    let keyboard = build_admin_menu_users_keyboard(&users);
    bot.send_message(dialogue.chat_id(), T.admin_users.text)
        .reply_markup(keyboard)
        .await?;
    dialogue.update(CwDialogueState::AwaitingUser).await?;

    Ok(())
}

pub fn build_admin_menu_users_keyboard(users: &[UserDto]) -> InlineKeyboardMarkup {
    let buttons = users
        .iter()
        .filter(|u| u.full_name.is_some())
        .map(|user| build_user_inline_button(
            user.id, 
            user.full_name.as_ref().unwrap(),
        ))
        .collect::<Vec<_>>();

    InlineKeyboardMarkup::new(
        buttons.chunks(2)
            .map(|chunk| Vec::from(chunk))
            .collect::<Vec<_>>()
    )
}

pub fn build_user_inline_button(user_id: i64, full_name: &str) -> InlineKeyboardButton {
    InlineKeyboardButton::callback(
        full_name.to_string(),
        format!("admin_menu_user:{}", user_id),
    )
}
