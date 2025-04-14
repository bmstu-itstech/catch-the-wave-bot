use teloxide::prelude::*;
use teloxide::dispatching::dialogue::GetChatId;
use teloxide::types::ParseMode;

use crate::domain::use_cases::GetUserUseCase;
use crate::presentation::handlers::texts::T;
use crate::presentation::handlers::utils::{CwBotError, CwHandlerResult};


pub async fn handle_admin_menu_user_callback(
    bot: Bot,
    q: CallbackQuery,
    use_case: GetUserUseCase,
) -> CwHandlerResult {
    bot.answer_callback_query(&q.id).await?;

    let user_id: i64 = q.data.as_ref().unwrap()
        .split(":").last().unwrap()
        .parse()
        .unwrap();
    let user = use_case.execute(user_id).await
        .map_err(|err| CwBotError::External(err.into()))?;

    let text = if let Some(user_task) = user.user_task {
        T.admin_users.user_info_with_current_meeting(
            user.username.as_str(),
            &user.full_name,
            &user.group_name,
            &user.next_task_status,
            &user_task.state,
            &user_task.partner_username,
            &user.completed_quests,
        )
    } else {
        T.admin_users.user_info(
            user.username.as_str(),
            &user.full_name,
            &user.group_name,
            &user.next_task_status,
            &user.completed_quests,
        )
    };
    bot.send_message(q.chat_id().unwrap(), text)
        .parse_mode(ParseMode::Html)
        .await?;
    Ok(())
}
