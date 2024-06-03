use teloxide::prelude::*;
use teloxide::types::*;

use crate::{
    handlers::msg_handlers::reply_actions::*, Action, ActionImp, ReplyAction, REPLY_ACTION,
};

pub fn send_message_with_buttons(
    bot: &Bot,
    user: &User,
    text: &str,
    keyboard: &InlineKeyboardMarkup,
) {
    let bot = bot.clone();
    let user_id = user.id.clone();
    let keyboard = keyboard.clone();
    let text = text.to_owned();

    tokio::spawn(async move {
        let _ = bot
            .send_message(user_id, text)
            .reply_markup(keyboard)
            .parse_mode(ParseMode::Html)
            .await
            .map_err(|e| error!("Error {}", e));
    });
}

pub async fn send_message_force_reply_update_action(
    bot: &Bot,
    user: &User,
    msg_to_send: &str,
    msg_reply: &str,
    reply_action: ReplyAction,
) {
    let msg_to_send = msg_to_send.to_string();
    let msg_reply = ForceReply::new().input_field_placeholder(msg_reply.to_owned());

    let bot = bot.clone();
    let user_id = user.id.clone();

    match bot
        .send_message(user_id, msg_to_send)
        .reply_markup(msg_reply)
        .parse_mode(ParseMode::MarkdownV2)
        .await
    {
        Ok(sent_msg) => {
            let user_id = user.id.0 as i64;
            let updated_action = Action::new(sent_msg.id.0, reply_action);
            {
                let mut action_map = REPLY_ACTION.lock().unwrap();
                action_map.insert(user_id as i64, updated_action);
            }
        }
        Err(e) => {
            error!("send_message_force_reply_update_action : Err = {:?}", e);
            close_reply_action(user.id.0 as i64);
        }
    }
}

pub fn send_message(bot: &Bot, user: &User, msg_to_send: &str) {
    let msg_to_send = msg_to_send.to_string();

    let bot = bot.clone();
    let user_id = user.id.clone();

    tokio::spawn(async move {
        let _ = bot
            .send_message(user_id, msg_to_send)
            .await
            .map_err(|e| error!("Error {}", e));
    });
}
