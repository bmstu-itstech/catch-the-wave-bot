use teloxide::dispatching::dialogue::InMemStorage;
use teloxide::prelude::Dialogue;

use crate::core::fsm::CwDialogueState;

pub(crate) type CwDialogue = Dialogue<CwDialogueState, InMemStorage<CwDialogueState>>;
pub(crate) type CwHandlerResult = Result<(), Box<dyn std::error::Error + Send + Sync>>;
