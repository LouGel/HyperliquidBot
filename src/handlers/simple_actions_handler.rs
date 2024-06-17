use crate::bot::*;
use teloxide::prelude::*;
use teloxide::types::User;

pub async fn simple_action_handler(bot: &Bot, user: User, menu: Vec<&str>, msg: Message) {
    let msg_from_id = msg.id;

    match menu.get(1) {
        Some(&"CLOSE") => {
            delete_message(&bot, &user, &msg_from_id);
        }

        _ => {
            send_unexpected_callback_function_error(bot, &user, &menu.join("_"));
            return;
        }
    };
}
