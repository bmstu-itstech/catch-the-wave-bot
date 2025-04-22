use teloxide::prelude::*;
use teloxide::types::{InlineKeyboardButton, InlineKeyboardMarkup};

use crate::domain::use_cases::CheckAdminUseCase;
use crate::presentation::handlers::texts::T;
use crate::presentation::handlers::utils::{CwBotError, CwHandlerResult};


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

#[derive(Clone)]
pub enum MenuCallback {
    Users,
    CreateNextTask,
    AssignPartner,
    Complete,
}

pub fn build_admin_menu_keyboard() -> InlineKeyboardMarkup {
    InlineKeyboardMarkup::new(vec![
        vec![
            MenuCallback::CreateNextTask.into(),
            MenuCallback::AssignPartner.into(),
        ],
        vec![
            MenuCallback::Complete.into(),
            MenuCallback::Users.into(),
        ]
    ])
}

impl Into<InlineKeyboardButton> for MenuCallback {
    fn into(self) -> InlineKeyboardButton {
        match self {
            MenuCallback::Users => InlineKeyboardButton::callback(
                T.admin_menu.users_button, MenuCallback::Users,
            ),
            MenuCallback::CreateNextTask => InlineKeyboardButton::callback(
                T.admin_menu.create_next_task_button, MenuCallback::CreateNextTask,
            ),
            MenuCallback::AssignPartner => InlineKeyboardButton::callback(
                T.admin_menu.assign_partner_button, MenuCallback::AssignPartner,
            ),
            MenuCallback::Complete => InlineKeyboardButton::callback(
                T.admin_menu.verification_button, MenuCallback::Complete,
            ),
        }
    }
}

impl Into<String> for MenuCallback {
    fn into(self) -> String {
        match self {
            MenuCallback::Users          => "admin_menu_users".to_string(),
            MenuCallback::CreateNextTask => "admin_menu_create_next_task".to_string(),
            MenuCallback::AssignPartner  => "admin_menu_assign_partner".to_string(),
            MenuCallback::Complete       => "admin_menu_complete".to_string(),
        }
    }
}

impl TryFrom<String> for MenuCallback {
    type Error = ();

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "admin_menu_users"            => Ok(MenuCallback::Users),
            "admin_menu_create_next_task" => Ok(MenuCallback::CreateNextTask),
            "admin_menu_assign_partner"   => Ok(MenuCallback::AssignPartner),
            "admin_menu_complete"         => Ok(MenuCallback::Complete),
            _ => Err(()),
        }
    }
}
