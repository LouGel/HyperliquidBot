use crate::is_registered;
use crate::menus::main_menu;
use crate::types::Command;
use crate::utils::create_user;
use teloxide::prelude::*;
use teloxide::types::ParseMode;

use super::get_user_from_msg;

pub async fn commands_handler(bot: Bot, msg: Message, _cmd: Command) -> anyhow::Result<()> {
    let msg_id = msg.id;
    let user = get_user_from_msg(&msg)?;

    let bot_clone = bot.clone();

    if !is_registered(&user.id) {
        create_user(&msg).await;
        bot.send_message(
            user.id,
            "Successfully registered (or some other success message)",
        )
        .await?;
    };
    let (text, keyboard) = main_menu(user.id).await?;
    bot.send_message(user.id, text)
        .reply_markup(keyboard)
        .parse_mode(ParseMode::Html)
        .await?;
    // }
    // };
    bot_clone.delete_message(user.id, msg_id).await?;
    Ok(())
}
