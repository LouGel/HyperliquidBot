use crate::handlers::reply_actions::send_error_and_close;
use crate::menus::main_menu;
use crate::types::Command;
use crate::utils::create_user;
use crate::{is_registered, send_unexpected_error};
use teloxide::prelude::*;
use teloxide::types::ParseMode;

use super::get_user_from_msg;
use crate::bot::send_message_with_buttons;

pub async fn commands_handler(bot: Bot, msg: Message, _cmd: Command) -> anyhow::Result<()> {
    let msg_id = msg.id;
    let user = get_user_from_msg(&msg)?;

    let bot_clone = bot.clone();

    if !is_registered(&user.id) {
        debug!("Not  registered");
        create_user(&msg).await;
        bot.send_message(
            user.id,
            "Successfully registered (or some other success message)",
        )
        .await?;
    };
    debug!("After registered");
    let (text, keyboard) = main_menu(user.id).await.map_err(|e| {
        send_unexpected_error(&bot, &user, e.to_string());
        anyhow::anyhow!("")
    })?;
    debug!("After main_menu");
    send_message_with_buttons(&bot, &user, &text, &keyboard);

    bot_clone.delete_message(user.id, msg_id).await?;
    Ok(())
}
