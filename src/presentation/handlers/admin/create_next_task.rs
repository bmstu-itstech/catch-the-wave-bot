use teloxide::prelude::*;

use crate::domain::use_cases::{CheckNextTaskUseCase, CreateNextTaskUseCase};
use crate::presentation::handlers::fsm::CwDialogueState;
use crate::presentation::handlers::utils::{CwBotError, CwDialogue, CwHandlerResult};
use crate::presentation::handlers::texts::T;


pub async fn handle_admin_menu_create_next_task(
    bot: Bot,
    q: CallbackQuery,
    dialogue: CwDialogue,
    check_next_task_use_case: CheckNextTaskUseCase,
) -> CwHandlerResult {
    bot.answer_callback_query(q.id).await?;
    
    if check_next_task_use_case.execute().await
        .map_err(|err| CwBotError::Other(err.to_string()))?
    {
        bot.send_message(dialogue.chat_id(), T.admin_create_task.already_exists).await?;
        return Ok(());
    }
    
    bot.send_message(dialogue.chat_id(), T.admin_create_task.enter_title).await?;
    dialogue.update(CwDialogueState::AwaitingTaskTitle).await?;
    Ok(())
}

pub async fn receive_task_title(
    bot: Bot,
    msg: Message,
    dialogue: CwDialogue,
) -> CwHandlerResult {
    let title = msg.text().unwrap();
    bot.send_message(dialogue.chat_id(), T.admin_create_task.enter_description).await?;
    dialogue.update(CwDialogueState::AwaitingTaskDescription { title: title.to_string() }).await?;
    Ok(())
}

pub async fn receive_task_description(
    bot: Bot,
    msg: Message,
    dialogue: CwDialogue,
    create_next_task_use_case: CreateNextTaskUseCase,
    title: String,
) -> CwHandlerResult {
    let description = msg.text().unwrap();
    create_next_task_use_case.execute(&title, description).await
        .map_err(|err| CwBotError::Other(err.to_string()))?;
    bot.send_message(dialogue.chat_id(), T.admin_create_task.success).await?;
    dialogue.update(CwDialogueState::Idle).await?;
    Ok(())
}
