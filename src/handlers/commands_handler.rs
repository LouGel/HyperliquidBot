use crate::is_registered;
use crate::menus::*;
use crate::types::Command;
use crate::utils::create_user;
use crate::{bot::*, TOKEN_LIST};
use teloxide::prelude::*;

use super::get_user_from_msg;

pub async fn commands_handler(bot: Bot, msg: Message, cmd: Command) -> anyhow::Result<()> {
    let msg_id = msg.id;
    let user = get_user_from_msg(&msg)?;

    let menu = match cmd {
        Command::Start => {
            if !is_registered(&user.id) {
                if let None = create_user(&msg).await {
                    send_unexpected_error(&bot, &user, "Error with registration".to_owned());
                    return Err(anyhow::anyhow!("Error with registration"));
                }
                bot.send_message(user.id, "Successfully registered").await?;
            };
            main_menu(user.id).await
        }
        Command::UpdateTokens => {
            match TOKEN_LIST.refresh().await {
                Ok(()) => send_message(&bot, &user, "Token list refreshed"),
                Err(e) => send_error(&bot, &user, &e.to_string()),
            };
            bot.delete_message(user.id, msg_id).await?;
            return Ok(());
        }
        Command::Balances => balance_menu(&user).await,
        Command::Settings => settings_menu().await,
        Command::TradeMenu => trade_menu().await,
    };
    match menu {
        Err(e) => send_unexpected_error(&bot, &user, e.to_string()),
        Ok((text, keyboard)) => send_message_with_buttons(&bot, &user, &text, &keyboard),
    }
    delete_message(&bot, &user, &msg_id);
    Ok(())
}
