use teloxide::utils::command::BotCommands;

#[derive(Clone, BotCommands)]
pub enum Command {
    #[command(rename="cancel")]
    Cancel,
    
    #[command(rename="start")]
    Start,
    
    #[command(rename="current")]
    CurrentMeeting,
    
    #[command(rename="next")]
    NextMeeting,

    #[command(rename="register")]
    ReRegister,
    
    #[command(rename="rules")]
    Rules,
}
