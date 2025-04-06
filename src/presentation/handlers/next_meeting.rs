use teloxide::prelude::*;
use teloxide::types::{KeyboardButton, KeyboardMarkup};

use crate::core::fsm::CwDialogueState;
use crate::domain::use_cases::{AcceptMeetingUseCase, GetMenuStateUseCase, GetNextMeetingError, GetNextMeetingUseCase, RejectMeetingUseCase};
use crate::presentation::handlers::menu::send_menu;

use super::texts::T;
use super::utils::{CwBotError, CwDialogue, CwHandlerResult};


pub async fn handle_next_meeting_callback(
    bot: Bot,
    q: CallbackQuery,
    dialogue: CwDialogue,
    use_case: GetNextMeetingUseCase,
) -> CwHandlerResult {
    bot.answer_callback_query(q.id).await?;
    
    let next_meeting = match use_case.execute(dialogue.chat_id().0).await {
        Ok(next_meeting) => next_meeting,
        Err(err) => return match err {
            GetNextMeetingError::UserNotFound(_) => Err(CwBotError::Other(err.to_string())),
            GetNextMeetingError::ServiceError(e) => Err(e.into()),
        }
    };

    if next_meeting.is_none() {
        bot.send_message(dialogue.chat_id(), T.meeting.no_next_meeting).await?;
        return Ok(());
    }

    bot.send_message(dialogue.chat_id(), T.meeting.accept_next_meeting)
        .reply_markup(next_meeting_keyboard())
        .await?;

    dialogue.update(CwDialogueState::AwaitingAcceptNextMeeting).await?;

    Ok(())
}

pub async fn handle_next_meeting_accept(
    bot: Bot,
    msg: Message,
    dialogue: CwDialogue,
    accept_meeting_use_case: AcceptMeetingUseCase,
    get_menu_state_use_case: GetMenuStateUseCase,
) -> CwHandlerResult {
    if let Err(err) = accept_meeting_use_case.execute(msg.chat.id.0).await {
        return Err(CwBotError::Other(format!("{:?}", err)));
    }

    bot.send_message(msg.chat.id, T.meeting.after_accept).await?;
    dialogue.update(CwDialogueState::Idle).await?;
    log::info!("user {} accepted the next meeting", msg.chat.username().unwrap());
    
    send_menu(bot, msg, get_menu_state_use_case).await
}

pub async fn handle_next_meeting_reject(
    bot: Bot,
    msg: Message,
    dialogue: CwDialogue,
    reject_meeting_use_case: RejectMeetingUseCase,
    get_menu_state_use_case: GetMenuStateUseCase,
) -> CwHandlerResult {
    if let Err(err) = reject_meeting_use_case.execute(msg.chat.id.0).await {
        return Err(CwBotError::Other(format!("{:?}", err)));
    }

    bot.send_message(msg.chat.id, T.meeting.after_reject).await?;
    dialogue.update(CwDialogueState::Idle).await?;
    log::info!("user {} rejected the next meeting", msg.chat.username().unwrap());
    
    send_menu(bot, msg, get_menu_state_use_case).await
}

#[derive(Debug, Clone)]
pub enum NextMeeting {
    Accept,
    Reject,
}

impl Into<String> for NextMeeting {
    fn into(self) -> String {
        match self {
            NextMeeting::Accept => String::from(T.meeting.accept_button),
            NextMeeting::Reject => String::from(T.meeting.reject_button),
        }
    }
}

impl Into<KeyboardButton> for NextMeeting {
    fn into(self) -> KeyboardButton {
        KeyboardButton::new::<String>(self.into())
    }
}

impl TryFrom<String> for NextMeeting {
    type Error = ();

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() { 
            val if val == T.meeting.accept_button => Ok(NextMeeting::Accept),
            val if val == T.meeting.reject_button => Ok(NextMeeting::Reject),
            _ => Err(()),
        }
    }
}

pub fn next_meeting_keyboard() -> KeyboardMarkup {
    KeyboardMarkup::new(vec![
        vec![
            NextMeeting::Accept.into(),
            NextMeeting::Reject.into(),
        ],
    ])
        .one_time_keyboard()
}
