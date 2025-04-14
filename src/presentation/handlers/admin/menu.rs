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
pub enum AdminMenuCallback {
    Users,
    Meetings,
    AssignPartner,
    Verify,
}

pub fn build_admin_menu_keyboard() -> InlineKeyboardMarkup {
    InlineKeyboardMarkup::new(vec![
        vec![
            AdminMenuCallback::Users.into(),
            AdminMenuCallback::Meetings.into(),
        ],
        vec![
            AdminMenuCallback::AssignPartner.into(),
            AdminMenuCallback::Verify.into(),
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
            ),
            AdminMenuCallback::AssignPartner => InlineKeyboardButton::callback(
                T.admin_menu.assign_partner_button, AdminMenuCallback::AssignPartner,
            ),
            AdminMenuCallback::Verify => InlineKeyboardButton::callback(
                T.admin_menu.verification_button, AdminMenuCallback::Verify,
            ),
        }
    }
}

impl Into<String> for AdminMenuCallback {
    fn into(self) -> String {
        match self {
            AdminMenuCallback::Users => "admin_menu_users".to_string(),
            AdminMenuCallback::Meetings => "admin_menu_meetings".to_string(),
            AdminMenuCallback::AssignPartner => "admin_menu_assign_partner".to_string(),
            AdminMenuCallback::Verify => "admin_menu_verify".to_string(),
        }
    }
}

impl TryFrom<String> for AdminMenuCallback {
    type Error = ();

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "admin_menu_users" => Ok(AdminMenuCallback::Users),
            "admin_menu_meetings" => Ok(AdminMenuCallback::Meetings),
            "admin_menu_assign_partner" => Ok(AdminMenuCallback::AssignPartner),
            "admin_menu_verify" => Ok(AdminMenuCallback::Verify),
            _ => Err(()),
        }
    }
}
