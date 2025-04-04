use std::fmt::Debug;
use teloxide::dispatching::dialogue::{InMemStorage, InMemStorageError};
use teloxide::prelude::Dialogue;
use teloxide::RequestError;

use crate::core::fsm::CwDialogueState;

pub(crate) type CwDialogue = Dialogue<CwDialogueState, InMemStorage<CwDialogueState>>;
pub(crate) type CwHandlerResult = Result<(), CwBotError>;


#[derive(Debug)]
pub enum CwBotError {
    Telegram(RequestError),
    InMemStorage(InMemStorageError),
    External(Box<dyn std::error::Error + Send + Sync>),
    Other(String),
}

impl From<RequestError> for CwBotError {
    fn from(e: RequestError) -> Self {
        CwBotError::Telegram(e)
    }
}

impl From<Box<dyn std::error::Error + Send + Sync>> for CwBotError {
    fn from(e: Box<dyn std::error::Error + Send + Sync>) -> Self {
        CwBotError::External(e)
    }
}

impl From<InMemStorageError> for CwBotError {
    fn from(e: InMemStorageError) -> Self {
        CwBotError::InMemStorage(e)
    }
}
