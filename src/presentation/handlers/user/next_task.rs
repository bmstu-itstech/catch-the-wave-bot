use teloxide::prelude::*;
use teloxide::types::{KeyboardButton, KeyboardMarkup};

use crate::domain::use_cases::{AcceptNextTaskUseCase, GetMenuStateUseCase, RejectTaskUseCase};
use crate::presentation::handlers::fsm::CwDialogueState;
use crate::presentation::handlers::texts::T;
use crate::presentation::handlers::user::send_menu;
use crate::presentation::handlers::utils::{CwBotError, CwDialogue, CwHandlerResult};


pub async fn handle_next_meeting_callback(
    bot: Bot,
    q: CallbackQuery,
    dialogue: CwDialogue,
) -> CwHandlerResult {
    bot.answer_callback_query(q.id).await?;
    
    bot.send_message(dialogue.chat_id(), T.next_task.text)
        .reply_markup(next_meeting_keyboard())
        .await?;

    dialogue.update(CwDialogueState::AwaitingAcceptNextMeeting).await?;

    Ok(())
}

pub async fn handle_next_meeting_accept(
    bot: Bot,
    msg: Message,
    dialogue: CwDialogue,
    accept_task_use_case: AcceptNextTaskUseCase,
    get_menu_state_use_case: GetMenuStateUseCase,
) -> CwHandlerResult {
    accept_task_use_case.execute(msg.chat.id.0).await
        .map_err(|err| CwBotError::Other(err.to_string()))?;

    bot.send_message(msg.chat.id, T.next_task.accept_success).await?;
    dialogue.update(CwDialogueState::Idle).await?;
    log::info!("user @{} accepted the next task", msg.chat.username().unwrap());
    
    send_menu(bot, msg, get_menu_state_use_case).await
}

pub async fn handle_next_meeting_reject(
    bot: Bot,
    msg: Message,
    dialogue: CwDialogue,
    reject_task_use_case: RejectTaskUseCase,
    get_menu_state_use_case: GetMenuStateUseCase,
) -> CwHandlerResult {
    reject_task_use_case.execute(msg.chat.id.0).await
        .map_err(|err| CwBotError::Other(err.to_string()))?;

    bot.send_message(msg.chat.id, T.next_task.reject_success).await?;
    dialogue.update(CwDialogueState::Idle).await?;
    log::info!("user @{} rejected the next task", msg.chat.username().unwrap());

    send_menu(bot, msg, get_menu_state_use_case).await

}

#[derive(Debug, Clone)]
pub enum NextTaskCallback {
    Accept,
    Reject,
}

impl Into<String> for NextTaskCallback {
    fn into(self) -> String {
        match self {
            NextTaskCallback::Accept => String::from(T.next_task.accept_button),
            NextTaskCallback::Reject => String::from(T.next_task.reject_button),
        }
    }
}

impl Into<KeyboardButton> for NextTaskCallback {
    fn into(self) -> KeyboardButton {
        KeyboardButton::new::<String>(self.into())
    }
}

impl TryFrom<String> for NextTaskCallback {
    type Error = ();

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() { 
            val if val == T.next_task.accept_button => Ok(NextTaskCallback::Accept),
            val if val == T.next_task.reject_button => Ok(NextTaskCallback::Reject),
            _ => Err(()),
        }
    }
}

pub fn next_meeting_keyboard() -> KeyboardMarkup {
    KeyboardMarkup::new(vec![
        vec![
            NextTaskCallback::Accept.into(),
            NextTaskCallback::Reject.into(),
        ],
    ])
        .one_time_keyboard()
}
