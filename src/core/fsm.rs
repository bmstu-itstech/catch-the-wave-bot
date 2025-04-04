use serde::{Serialize, Deserialize};

#[derive(Default, Clone)] 
#[derive(Serialize, Deserialize)]
pub enum CwDialogueState {
    #[default]
    Idle,
    
    AwaitingFullName,
    AwaitingGroupName { full_name: String },

    Menu,
    AwaitingAcceptNextMeeting,
}
