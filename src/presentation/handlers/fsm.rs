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
    
    AwaitingUser,
    AwaitingPartner1,
    AwaitingPartner2 { partner_1_id: i64 },
    
    AwaitingTaskTitle,
    AwaitingTaskDescription { title: String },

    AwaitingUserForComplete,
}
