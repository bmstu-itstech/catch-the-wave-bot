use teloxide::prelude::*;

use crate::domain::use_cases::{AssignPartnerUseCase, GetFreeUsersUseCase};
use crate::presentation::handlers::fsm::CwDialogueState;
use crate::presentation::handlers::texts::T;
use crate::presentation::handlers::utils::{CwBotError, CwDialogue, CwHandlerResult};

use super::build_admin_menu_users_keyboard;


pub async fn handle_admin_menu_assign_partner_callback(
    bot: Bot,
    q: CallbackQuery,
    dialogue: CwDialogue,
    use_case: GetFreeUsersUseCase,
) -> CwHandlerResult {
    bot.answer_callback_query(&q.id).await?;
    
    let users = use_case.execute().await
        .map_err(|err| CwBotError::External(err.into()))?;

    if users.len() < 2 {
        bot.send_message(dialogue.chat_id(), T.admin_assign.insufficient_users)
            .await?;
        return Ok(());
    }
    
    let keyboard = build_admin_menu_users_keyboard(&users);
    bot.send_message(dialogue.chat_id(), T.admin_assign.assign_first)
        .reply_markup(keyboard)
        .await?;
    dialogue.update(CwDialogueState::AwaitingPartner1).await?;

    Ok(())

}

pub async fn handle_admin_menu_assign_partner_1_callback(
    bot: Bot,
    q: CallbackQuery,
    dialogue: CwDialogue,
    use_case: GetFreeUsersUseCase,
) -> CwHandlerResult {
    bot.answer_callback_query(&q.id).await?;

    let partner_1_id: i64 = q.data.as_ref().unwrap()
        .split(":").last().unwrap()
        .parse()
        .unwrap();

    let users = use_case.execute().await
        .map_err(|err| CwBotError::External(err.into()))?;
    let users = users.into_iter().filter(|user| user.id != partner_1_id).collect::<Vec<_>>();

    let keyboard = build_admin_menu_users_keyboard(&users);
    bot.send_message(dialogue.chat_id(), T.admin_assign.assign_second)
        .reply_markup(keyboard)
        .await?;
    
    dialogue.update(CwDialogueState::AwaitingPartner2 { partner_1_id }).await?;
    
    Ok(())
}

pub async fn handle_admin_menu_assign_partner_2_callback(
    bot: Bot,
    q: CallbackQuery,
    dialogue: CwDialogue,
    use_case: AssignPartnerUseCase,
    partner_1_id: i64,
) -> CwHandlerResult {
    bot.answer_callback_query(&q.id).await?;

    let partner_2_id: i64 = q.data.as_ref().unwrap()
        .split(":").last().unwrap()
        .parse()
        .unwrap();

    use_case.execute(partner_1_id, partner_2_id).await
        .map_err(|err| CwBotError::External(err.into()))?;
    
    dialogue.update(CwDialogueState::Idle).await?;

    bot.send_message(dialogue.chat_id(), T.admin_assign.assign_success)
        .await?;
    
    Ok(())
}
