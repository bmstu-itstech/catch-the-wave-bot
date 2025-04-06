use teloxide::utils::command::BotCommands;

#[derive(Clone, BotCommands)]
pub enum Command {
    #[command(rename = "cancel")]
    Cancel,

    #[command(rename = "start")]
    Start,
    
    #[command(rename="admin")]
    Admin,
}
