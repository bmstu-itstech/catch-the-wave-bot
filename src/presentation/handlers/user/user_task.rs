use teloxide::prelude::*;

use crate::domain::use_cases::{GetUserTaskUseCase, GetMenuStateUseCase};
use crate::presentation::handlers::texts::T;
use crate::presentation::handlers::user::menu::send_menu_callback;
use crate::presentation::handlers::utils::{CwBotError, CwDialogue, CwHandlerResult};


pub async fn handle_current_meeting_callback(
    bot: Bot,
    q: CallbackQuery,
    dialogue: CwDialogue,
    get_user_task_use_case: GetUserTaskUseCase,
    get_menu_state_use_case: GetMenuStateUseCase,
) -> CwHandlerResult {
    bot.answer_callback_query(&q.id).await?;

    let user_task = get_user_task_use_case.execute(dialogue.chat_id().0).await
        .map_err(|err| CwBotError::Other(err.to_string()))?;

    bot.send_message(dialogue.chat_id(), T.user_task.user_task(
        &user_task.partner_username,
        &user_task.title,
        &user_task.description,
    )).await?;

    send_menu_callback(bot, q, get_menu_state_use_case).await
}
