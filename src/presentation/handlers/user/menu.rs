use teloxide::dispatching::dialogue::GetChatId;
use teloxide::prelude::*;
use teloxide::types::{InlineKeyboardButton, InlineKeyboardMarkup};

use crate::domain::use_cases::{GetMenuStateUseCase, MenuCategory, MenuState};
use crate::presentation::handlers::texts::T;
use crate::presentation::handlers::utils::{CwBotError, CwHandlerResult};

pub async fn send_menu(
    bot: Bot,
    msg: Message,
    use_case: GetMenuStateUseCase,
) -> CwHandlerResult {
    send_menu_in_chat(bot, msg.chat.id, use_case).await
}

pub async fn send_menu_callback(
    bot: Bot,
    q: CallbackQuery,
    use_case: GetMenuStateUseCase,
) -> CwHandlerResult {
    send_menu_in_chat(bot, q.chat_id().unwrap(), use_case).await
}

async fn send_menu_in_chat(
    bot: Bot,
    chat_id: ChatId,
    use_case: GetMenuStateUseCase,
) -> CwHandlerResult {
    let menu_state = use_case.execute(chat_id.0).await
        .map_err(|err| CwBotError::Other(err.to_string()))?;

    let keyboard = build_keyboard(menu_state);

    bot.send_message(chat_id, T.menu.text)
        .reply_markup(keyboard)
        .await?;
    Ok(())
}

#[derive(Clone)]
pub enum MenuCallback {
    Profile,
    Rules,
    NextTask,
    UserTask,
}

pub fn build_keyboard(state: MenuState) -> InlineKeyboardMarkup {
    let mut rows: Vec<Vec<InlineKeyboardButton>> = Vec::new();
    
    let mut first_row: Vec<InlineKeyboardButton> = Vec::new();
    if state.categories.contains(&MenuCategory::Profile) {
        first_row.push(MenuCallback::Profile.into());
    }
    
    if state.categories.contains(&MenuCategory::Rules) {
        first_row.push(MenuCallback::Rules.into());
    }
    
    if !first_row.is_empty() {
        rows.push(first_row);
    }
    
    let mut second_row: Vec<InlineKeyboardButton> = Vec::new();
    if state.categories.contains(&MenuCategory::NextTask) {
        second_row.push(MenuCallback::NextTask.into());
    }
    
    if state.categories.contains(&MenuCategory::UserTask) {
        second_row.push(MenuCallback::UserTask.into());
    }
    
    if !second_row.is_empty() {
        rows.push(second_row);
    }
    
    InlineKeyboardMarkup::new(rows)
}

impl Into<InlineKeyboardButton> for MenuCallback {
    fn into(self) -> InlineKeyboardButton {
        match self {
            MenuCallback::Profile => InlineKeyboardButton::callback(
                T.menu.profile_button, MenuCallback::Profile,
            ),
            MenuCallback::Rules => InlineKeyboardButton::callback(
                T.menu.rules_button, MenuCallback::Rules,
            ),
            MenuCallback::NextTask => InlineKeyboardButton::callback(
                T.menu.next_task_button, MenuCallback::NextTask,
            ),
            MenuCallback::UserTask => InlineKeyboardButton::callback(
                T.menu.user_task_button, MenuCallback::UserTask,
            )
        }
    }
}

impl Into<String> for MenuCallback {
    fn into(self) -> String {
        match self {
            MenuCallback::Profile  => "menu_profile".to_string(),
            MenuCallback::Rules    => "menu_rules".to_string(),
            MenuCallback::NextTask => "menu_next_task".to_string(),
            MenuCallback::UserTask => "menu_user_task".to_string(),
        }
    }
}

impl TryFrom<String> for MenuCallback {
    type Error = ();

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "menu_profile"   => Ok(MenuCallback::Profile),
            "menu_rules"     => Ok(MenuCallback::Rules),
            "menu_next_task" => Ok(MenuCallback::NextTask),
            "menu_user_task" => Ok(MenuCallback::UserTask),
            _ => Err(()),
        }
    }
}
