use teloxide::dispatching::dialogue::GetChatId;
use teloxide::prelude::*;
use teloxide::types::{InlineKeyboardButton, InlineKeyboardMarkup};

use crate::domain::use_cases::{CheckAdminUseCase, GetAllUsersUseCase};

use super::texts::T;
use super::utils::{CwBotError, CwHandlerResult};


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
    
    let usernames = users
        .iter()
        .map(|user| user.username.as_str())
        .collect::<Vec<_>>();
    
    bot.send_message(q.chat_id().unwrap(), T.admin_menu.users_query_text(usernames.as_slice())).await?;
    
    Ok(())
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
