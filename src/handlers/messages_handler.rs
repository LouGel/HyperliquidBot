use crate::bot::*;

use crate::create_user;
use crate::globals::*;
use crate::handlers::message_handler_reply;
use crate::is_registered;
use anyhow::anyhow;
use teloxide::prelude::*;

use teloxide::types::{MessageKind, User};
pub fn get_user_from_msg(msg: &Message) -> anyhow::Result<User> {
    msg.from()
        .ok_or(anyhow!("Could get user from : {:?}", msg))
        .cloned()
}
pub async fn message_handler(bot: Bot, msg: Message) -> anyhow::Result<()> {
    let user = get_user_from_msg(&msg)?;
    if !is_registered(&user.id) {
        create_user(&msg).await;
        bot.send_message(
            user.id,
            "Successfully registered (or some other success message)",
        )
        .await?;
    } else if let MessageKind::Common(msg_common) = msg.clone().kind {
        if let Some(reply_to) = msg_common.reply_to_message {
            parse_message_reply(&bot, &msg, &user, &reply_to)?;
        }
    }
    delete_message(&bot, &user, &msg.id);
    Ok(())
}
pub fn parse_message_reply(
    bot: &Bot,
    msg: &Message,
    user_from: &User,
    msg_from: &Message,
) -> anyhow::Result<()> {
    let user_id_number = user_from.id.0 as i64;
    let msg_from_id = msg_from.id.0;
    let bot_clone = bot.clone();

    if let Some(resp_action) = {
        let action_map = REPLY_ACTION.lock().unwrap();
        action_map.get(&(user_id_number)).cloned()
    } {
        if resp_action.msg_id == msg_from_id {
            message_handler_reply(&bot, &msg, &user_from, resp_action)?;
        } else {
            error!("User {user_id_number} reply to an unregistered msg{msg_from_id}");
            send_error(&bot, &user_from, "This dialogue is closed");
            delete_message(&bot, &user_from, &msg_from.id);
        }
    } else {
        error!("User {user_id_number} reply to an unregistered msg{msg_from_id}");
        send_error(
            &bot,
            &user_from,
            "Bot restarted : please retry the function",
        );
    }
    delete_message(&bot_clone, &user_from, &msg_from.id);

    Ok(())
}
