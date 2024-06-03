use ethers_core::rand::thread_rng;
use ethers_core::rand::RngCore;
use teloxide::prelude::*;
use teloxide::types::*;

pub fn send_error(bot: &Bot, user: &User, err_msg: &str) {
    let format_err = format!("⚠️ Error: {err_msg}⚠️");
    let bot = bot.clone();
    let user_id = user.id.clone();

    tokio::spawn(async move {
        let _ = bot
            .send_message(user_id, format_err)
            .parse_mode(ParseMode::Html)
            .await
            .map_err(|e| error!("Error {}", e));
    });
}

pub fn send_unexpected_error(bot: &Bot, user: &User, error: String) {
    let user_id = user.id;
    let mut rng = thread_rng();
    let aleatory: u64 = rng.next_u64();
    let format_err = format!(
        "⚠️ Unexpected Error: Retry or ask suppor with ref : {}:{:X}⚠️",
        user_id, aleatory
    );
    error!("Error no: {:X} for {user_id}. Value : \n {error}", aleatory);
    let bot = bot.clone();
    let user_id = user.id.clone();

    tokio::spawn(async move {
        let _ = bot
            .send_message(user_id, format_err)
            .parse_mode(ParseMode::Html)
            .await
            .map_err(|e| error!("Error {}", e));
    });
}
pub fn send_unexpected_callback_function_error(bot: &Bot, user: &User, callback: &str) {
    let user_id = user.id;
    let mut rng = thread_rng();
    let aleatory: u64 = rng.next_u64();
    let format_err = format!(
        "⚠️ Unexpected Error: Might be due to update, stop using  older messages and relaunch with /start if it persists , here the ref to send to the support: {}:{:X}⚠️",
        user_id, aleatory
    );
    error!(
        "Error no: {:X} for {user_id}.  Callback = {callback}",
        aleatory
    );
    let bot = bot.clone();
    let user_id = user.id.clone();

    tokio::spawn(async move {
        let _ = bot
            .send_message(user_id, format_err)
            .parse_mode(ParseMode::Html)
            .await
            .map_err(|e| error!("Error {}", e));
    });
}
