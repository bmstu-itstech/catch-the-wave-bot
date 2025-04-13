use teloxide::prelude::*;
use crate::domain::error::DomainError;
use crate::domain::use_cases::{GetCurrentMeetingUseCase, GetMenuStateUseCase};
use crate::presentation::handlers::menu::send_menu_callback;
use super::texts::T;
use super::utils::{CwBotError, CwDialogue, CwHandlerResult};


pub async fn handle_current_meeting_callback(
    bot: Bot,
    q: CallbackQuery,
    dialogue: CwDialogue,
    get_current_meeting_use_case: GetCurrentMeetingUseCase,
    get_menu_state_use_case: GetMenuStateUseCase,
) -> CwHandlerResult {
    bot.answer_callback_query(&q.id).await?;

    let current_meeting = match get_current_meeting_use_case.execute(dialogue.chat_id().0).await {
        Ok(current_meeting) => current_meeting,
        Err(err) => return match err {
            DomainError::Other(e) => Err(e.into()),
            _ => Err(CwBotError::Other(err.to_string())),
        }
    };

    bot.send_message(dialogue.chat_id(), T.meeting.current_meeting_text(
        &current_meeting.quest_text,
        &current_meeting.partner_username,
    )).await?;

    send_menu_callback(bot, q, get_menu_state_use_case).await
}
