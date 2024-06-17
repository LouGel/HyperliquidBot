use crate::init::update_token_list;
use crate::menus::main_menu;
use crate::types::Command;
use crate::utils::create_user;
use crate::{is_registered, send_unexpected_error};
use teloxide::prelude::*;

use super::get_user_from_msg;
use crate::bot::send_message_with_buttons;

pub async fn commands_handler(bot: Bot, msg: Message, cmd: Command) -> anyhow::Result<()> {
    let msg_id = msg.id;
    let user = get_user_from_msg(&msg)?;

    let bot_clone = bot.clone();

    match cmd {
        Command::Start => {
            if !is_registered(&user.id) {
                create_user(&msg).await;
                bot.send_message(
                    user.id,
                    "Successfully registered (or some other success message)",
                )
                .await?;
            };

            let (text, keyboard) = main_menu(user.id).await.map_err(|e| {
                send_unexpected_error(&bot, &user, e.to_string());
                anyhow::anyhow!("")
            })?;

            send_message_with_buttons(&bot, &user, &text, &keyboard);

            bot_clone.delete_message(user.id, msg_id).await?;
        }
        Command::UpdateTokens => update_token_list().await?,
    }
    Ok(())
}
