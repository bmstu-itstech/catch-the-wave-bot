#[derive(Debug, Clone)]
pub enum CurrentMeetingState {
    Active,
    Verified,
}

#[derive(Debug, Clone)]
pub struct CurrentMeeting {
    pub state: CurrentMeetingState,
    pub partner_id: i64,
}

impl CurrentMeeting {
    pub fn new(state: CurrentMeetingState, partner_id: i64) -> Self {
        CurrentMeeting { state, partner_id }
    }
}
