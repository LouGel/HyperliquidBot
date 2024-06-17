use teloxide::utils::command::BotCommands;
#[derive(BotCommands, Clone)]
#[command(
    rename_rule = "lowercase",
    description = "These commands are supported:"
)]
pub enum Command {
    #[command(description = "handle a start.")]
    Start,
    #[command(description = "update_tokens.")]
    UpdateTokens,
}
