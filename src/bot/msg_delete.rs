use teloxide::prelude::*;
use teloxide::types::*;

pub fn delete_message(bot: &Bot, user: &User, msg_id: &MessageId) {
    let bot = bot.clone(); // no arc as per doc
    let user_id = user.id.clone();
    let msg_id = msg_id.clone();

    tokio::spawn(async move {
        let _ = bot
            .delete_message(user_id, msg_id)
            .await
            .map_err(|e| error!("Error {}", e));
    });
}
