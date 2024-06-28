use teloxide::utils::command::BotCommands;
#[derive(BotCommands, Clone)]
#[command(
    rename_rule = "lowercase",
    description = "These commands are supported:"
)]
pub enum Command {
    #[command(description = "Handle a Start.")]
    Start,
    #[command(description = "Update tokens list.")]
    UpdateTokens,
    #[command(description = "Trade Menu")]
    TradeMenu,
    #[command(description = "Wallets settngs")]
    Settings,
    #[command(description = "Get Balances")]
    Balances,
}
