use teloxide::prelude::*;

use crate::presentation::handlers::texts::T;
use crate::presentation::handlers::utils::{CwDialogue, CwHandlerResult};


pub async fn handle_rules_callback(
    bot: Bot,
    q: CallbackQuery,
    dialogue: CwDialogue,
) -> CwHandlerResult {
    bot.answer_callback_query(&q.id).await?;
    bot.send_message(dialogue.chat_id(), T.rules.text).await?;
    Ok(())
}
