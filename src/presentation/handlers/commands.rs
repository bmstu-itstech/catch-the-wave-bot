use teloxide::utils::command::BotCommands;

#[derive(Clone, BotCommands)]
pub enum Command {
    #[command(rename="start")]
    Start,
    //Cancel,
}
