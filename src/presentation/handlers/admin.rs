use chrono::{Datelike, Duration, NaiveDateTime, Weekday};
use chrono::Utc;

use teloxide::dispatching::dialogue::GetChatId;
use teloxide::prelude::*;
use teloxide::types::{InlineKeyboardButton, InlineKeyboardMarkup, ParseMode};

use crate::core::fsm::CwDialogueState;
use crate::domain::models::User;
use crate::domain::use_cases::{CheckAdminUseCase, CreateQuestUseCase, GetAllUsersUseCase, GetUserUseCase};

use super::texts::T;
use super::utils::{CwBotError, CwDialogue, CwHandlerResult};


pub async fn handle_admin_command(
    bot: Bot,
    msg: Message,
    check_admin_use_case: CheckAdminUseCase,    
) -> CwHandlerResult {
    let is_admin = check_admin_use_case.execute(msg.chat.id.0).await
        .map_err(|err| CwBotError::External(err.into()))?;
    
    if !is_admin {
        log::info!("user {0} made an attempt to call /admin command", msg.chat.id.0);
        return Ok(());
    }
    
    bot.send_message(msg.chat.id, T.admin_menu.text)
        .reply_markup(build_admin_menu_keyboard())
        .await?;
    Ok(())
}

pub async fn handle_admin_menu_users_callback(
    bot: Bot,
    q: CallbackQuery,
    use_case: GetAllUsersUseCase,
) -> CwHandlerResult {
    bot.answer_callback_query(&q.id).await?;

    let users = use_case.execute().await
        .map_err(|err| CwBotError::External(err.into()))?;

    let keyboard = build_admin_menu_users_keyboard(&users);
    bot.send_message(q.chat_id().unwrap(), T.admin_menu.users_query_header)
        .reply_markup(keyboard)
        .await?;

    Ok(())
}

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

    bot.send_message(q.chat_id().unwrap(), T.admin_menu.user_info_text(
        user.username.as_str(),
        user.profile.as_ref().unwrap().full_name.as_str(),
        user.profile.as_ref().unwrap().group_name.as_str(),
    )).await?;

    Ok(())
}

pub async fn handle_admin_menu_meetings_callback(
    bot: Bot,
    q: CallbackQuery,
) -> CwHandlerResult {
    bot.answer_callback_query(&q.id).await?;

    bot.send_message(q.chat_id().unwrap(), T.admin_menu.meetings_text)
        .reply_markup(build_admin_menu_meetings_keyboard())
        .await?;
    Ok(())
}

/*pub async fn handle_admin_menu_quest_callback(
    bot: Bot,
    q: CallbackQuery,
    get_current_meeting_use_case: GetCurrentQuestUseCase,
    get_next_meeting_use_case: GetNextQuestUseCase,
) -> CwHandlerResult {
    bot.answer_callback_query(&q.id).await?;

    let current_quest_text = get_current_meeting_use_case.execute().await
        .map_err(|err| CwBotError::External(err.into()))?
        .map(|q| q.text);
    let next_quest_text = get_next_meeting_use_case.execute().await
        .map_err(|err| CwBotError::External(err.into()))?
        .map(|q| q.text);

    if next_quest_text.is_none() {
        bot.send_message(
            q.chat_id().unwrap(),
            T.admin_menu.quests_info(current_quest_text, next_quest_text)
        )
            .reply_markup(build_admin_menu_quests_keyboard())
            .parse_mode(ParseMode::Html)
            .await?;
    } else {
        bot.send_message(
            q.chat_id().unwrap(),
            T.admin_menu.quests_info(current_quest_text, next_quest_text)
        )
            .parse_mode(ParseMode::Html)
            .await?;
    }
    Ok(())
}*/

pub async fn handle_admin_menu_quest_create_callback(
    bot: Bot,
    q: CallbackQuery,
    dialogue: CwDialogue,
) -> CwHandlerResult {
    bot.answer_callback_query(&q.id).await?;
    bot.send_message(q.chat_id().unwrap(), T.admin_menu.quests_create_next_text).await?;
    dialogue.update(CwDialogueState::AwaitingQuestText).await?;
    Ok(())
}

pub async fn receive_admin_menu_quest_create_text(
    bot: Bot,
    message: Message,
    dialogue: CwDialogue,
    use_case: CreateQuestUseCase,
) -> CwHandlerResult {
    let text = message.text().unwrap();
    
    use_case.execute(text).await
        .map_err(|err| CwBotError::External(err.into()))?;
    
    bot.send_message(message.chat.id, T.admin_menu.quests_create_next_success).await?;
    dialogue.reset().await?;
    Ok(())
}



pub fn build_admin_menu_users_keyboard(users: &[User]) -> InlineKeyboardMarkup {
    let buttons = users
        .iter()
        .filter(|user| user.profile.is_some())
        .map(|user| build_user_inline_button(
            user.id, user.profile.as_ref().unwrap().full_name.as_str(),
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

#[derive(Clone)]
pub enum AdminMenuCallback {
    Users,
    Meetings,
}

pub fn build_admin_menu_keyboard() -> InlineKeyboardMarkup {
    InlineKeyboardMarkup::new(vec![
        vec![
            AdminMenuCallback::Users.into(),
            AdminMenuCallback::Meetings.into(),
        ],
    ])
}

impl Into<InlineKeyboardButton> for AdminMenuCallback {
    fn into(self) -> InlineKeyboardButton {
        match self {
            AdminMenuCallback::Users => InlineKeyboardButton::callback(
                T.admin_menu.users_button, AdminMenuCallback::Users,
            ),
            AdminMenuCallback::Meetings => InlineKeyboardButton::callback(
                T.admin_menu.meetings_button, AdminMenuCallback::Meetings,
            )
        }
    }
}

impl Into<String> for AdminMenuCallback {
    fn into(self) -> String {
        match self {
            AdminMenuCallback::Users => "admin_menu_users".to_string(),
            AdminMenuCallback::Meetings => "admin_menu_meetings".to_string(),
        }
    }
}

impl TryFrom<String> for AdminMenuCallback {
    type Error = ();

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "admin_menu_users" => Ok(AdminMenuCallback::Users),
            "admin_menu_meetings" => Ok(AdminMenuCallback::Meetings),
            _ => Err(()),
        }
    }
}

#[derive(Clone)]
pub enum AdminMenuMeetingsCallback {
    Quests,
    Assign,
    Verify,
    Statistics,
    Promote,
}

pub fn build_admin_menu_meetings_keyboard() -> InlineKeyboardMarkup {
    InlineKeyboardMarkup::new(vec![
        vec![
            AdminMenuMeetingsCallback::Quests.into(),
            AdminMenuMeetingsCallback::Assign.into(),
        ],
        vec![
            AdminMenuMeetingsCallback::Verify.into(),
            AdminMenuMeetingsCallback::Statistics.into(),
        ],
    ])
}

impl Into<InlineKeyboardButton> for AdminMenuMeetingsCallback {
    fn into(self) -> InlineKeyboardButton {
        match self {
            AdminMenuMeetingsCallback::Quests => InlineKeyboardButton::callback(
                T.admin_menu.meetings_quests_button, AdminMenuMeetingsCallback::Quests,
            ),
            AdminMenuMeetingsCallback::Assign => InlineKeyboardButton::callback(
                T.admin_menu.meetings_assign_button, AdminMenuMeetingsCallback::Assign,
            ),
            AdminMenuMeetingsCallback::Verify => InlineKeyboardButton::callback(
                T.admin_menu.meetings_verify_button, AdminMenuMeetingsCallback::Verify,
            ),
            AdminMenuMeetingsCallback::Statistics => InlineKeyboardButton::callback(
                T.admin_menu.meetings_statistics_button, AdminMenuMeetingsCallback::Statistics,
            ),
            AdminMenuMeetingsCallback::Promote => InlineKeyboardButton::callback(
                T.admin_menu.meetings_promote_button, AdminMenuMeetingsCallback::Promote,
            )
        }
    }
}

impl Into<String> for AdminMenuMeetingsCallback {
    fn into(self) -> String {
        match self {
            AdminMenuMeetingsCallback::Quests => "admin_menu_meetings_quests".to_string(),
            AdminMenuMeetingsCallback::Assign => "admin_menu_meetings_assign".to_string(),
            AdminMenuMeetingsCallback::Statistics => "admin_menu_meetings_statistics".to_string(),
            AdminMenuMeetingsCallback::Verify => "admin_menu_meetings_verify".to_string(),
            AdminMenuMeetingsCallback::Promote => "admin_menu_meetings_promote".to_string(),
        }
    }
}

impl TryFrom<String> for AdminMenuMeetingsCallback {
    type Error = ();

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "admin_menu_meetings_quests" => Ok(AdminMenuMeetingsCallback::Quests),
            "admin_menu_meetings_assign" => Ok(AdminMenuMeetingsCallback::Assign),
            "admin_menu_meetings_statistics" => Ok(AdminMenuMeetingsCallback::Statistics),
            "admin_menu_meetings_verify" => Ok(AdminMenuMeetingsCallback::Verify),
            "admin_menu_meetings_promote" => Ok(AdminMenuMeetingsCallback::Promote),
            _ => Err(()),
        }
    }
}

#[derive(Clone)]
pub enum AdminMenuQuestsCallback {
    CreateNext,
}

pub fn build_admin_menu_quests_keyboard() -> InlineKeyboardMarkup {
    InlineKeyboardMarkup::new(vec![
        vec![
            AdminMenuQuestsCallback::CreateNext.into(),
        ],
    ])
}

impl Into<InlineKeyboardButton> for AdminMenuQuestsCallback {
    fn into(self) -> InlineKeyboardButton {
        match self {
            AdminMenuQuestsCallback::CreateNext => InlineKeyboardButton::callback(
                T.admin_menu.quests_create_next_button, AdminMenuMeetingsCallback::Quests,
            ),
        }
    }
}

impl Into<String> for AdminMenuQuestsCallback {
    fn into(self) -> String {
        match self {
            AdminMenuQuestsCallback::CreateNext => "admin_menu_quests_create_next".to_string(),
        }
    }
}

impl TryFrom<String> for AdminMenuQuestsCallback {
    type Error = ();

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "admin_menu_quests_create_next" => Ok(AdminMenuQuestsCallback::CreateNext),
            _ => Err(()),
        }
    }
}
