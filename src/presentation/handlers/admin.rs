use teloxide::dispatching::dialogue::GetChatId;
use teloxide::prelude::*;
use teloxide::types::{InlineKeyboardButton, InlineKeyboardMarkup};

use crate::domain::models::User;
use crate::domain::use_cases::{CheckAdminUseCase, FindUserByUsernameUseCase, GetAllUsersUseCase};

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

    let keyboard = build_admin_menu_users_keyboard(&users);
    bot.send_message(q.chat_id().unwrap(), T.admin_menu.users_query_header)
        .reply_markup(keyboard)
        .await?;
    
    Ok(())
}

pub async fn handle_admin_menu_user_callback(
    bot: Bot,
    q: CallbackQuery,
    use_case: FindUserByUsernameUseCase,
) -> CwHandlerResult {
    bot.answer_callback_query(&q.id).await?;
    
    let username = q.data.as_ref().unwrap().split(":").last().unwrap().to_string();
    let user = use_case.execute(&username).await
        .map_err(|err| CwBotError::External(err.into()))?;
    
    let user = match user {
        None => {
            bot.send_message(q.chat_id().unwrap(), T.admin_menu.user_not_found).await?;
            return Ok(())
        },
        Some(user) => user,
    };
    
    bot.send_message(q.chat_id().unwrap(), T.admin_menu.user_info_text(
        user.username.as_str(),
        user.profile.as_ref().unwrap().full_name.as_str(),
        user.profile.as_ref().unwrap().group_name.as_str(),
    )).await?;
    
    Ok(())
}

pub fn build_admin_menu_users_keyboard(users: &[User]) -> InlineKeyboardMarkup {
    let buttons = users
            .iter()
            .map(|user| build_user_inline_button(user))
            .collect::<Vec<_>>();
    
    InlineKeyboardMarkup::new(
        buttons.chunks(2)
            .map(|chunk| Vec::from(chunk))
            .collect::<Vec<_>>()
    )
}

pub fn build_user_inline_button(user: &User) -> InlineKeyboardButton {
    InlineKeyboardButton::callback(
        format!("@{}", user.username),
        format!("admin_menu_user:{}", user.username.as_str()), 
    )
}

/*pub async fn handle_admin_menu_users_pagination(
    bot: Bot,
    q: CallbackQuery,
    storage: PaginationStorage,
) -> CwHandlerResult {
    
}*/

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
