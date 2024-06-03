use teloxide::prelude::*;
use teloxide::types::*;

pub fn modify_message_with_buttons(
    bot: &Bot,
    user: &User,
    msg_id: MessageId,
    text: &str,
    keyboard: &InlineKeyboardMarkup,
) {
    let bot = bot.clone();
    let user_id = user.id.clone();
    let keyboard = keyboard.clone();
    let text = text.to_owned();

    tokio::spawn(async move {
        let _ = bot
            .edit_message_text(user_id, msg_id, text)
            .reply_markup(keyboard)
            .parse_mode(ParseMode::Html)
            .await
            .map_err(|e| error!("Error {}", e));
    });
}

pub fn modify_buttons(
    bot: &Bot,
    user_id: UserId,
    msg_id: MessageId,
    new_keyboard: InlineKeyboardMarkup,
) {
    let bot = bot.clone();

    tokio::spawn(async move {
        let _ = bot
            .edit_message_reply_markup(user_id, msg_id)
            .reply_markup(new_keyboard)
            .await
            .map_err(|e| error!("Error {}", e));
    });
}
