#[derive(Default, Debug, Clone)]
pub enum NextMeetingState {
    #[default]
    Pending,
    Accepted,
    Rejected,
}
