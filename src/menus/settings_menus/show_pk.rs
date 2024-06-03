use crate::{
    get_close_button, globals::*, is_passwd_set, send_message_force_reply_update_action,
    PKeyHandler,
};

use crate::types::*;

use crate::utils::keys_and_addresses::*;
use teloxide::prelude::*;
use teloxide::types::*;

pub fn pks(_bot: &Bot, user_id_number: i64) -> Option<(String, InlineKeyboardMarkup)> {
    let wallets_pks = {
        let pks_map = WALLETS_PKEY.lock().unwrap();
        pks_map.get(&user_id_number).cloned().unwrap()
    };
    let wallets_addresses = vec_3_p_keys_to_address(&wallets_pks);
    let text = format!(
        "<b>🤖 OmniBot X</b>\n\n\
        <b>═══ Your Wallets ═══</b> \n\n\
        <b>👝 Wallet⬩w1</b>\n\
        <b>Address: <code>{}</code></b>\n\
        <b>Private Key: <code>{}</code></b>\n\n\
        <b>👝 Wallet⬩w2</b>\n\
        <b>Address: <code>{}</code></b>\n\
        <b>Private Key: <code>{}</code></b>\n\n\
        <b>👝 Wallet⬩w3</b>\n\
        <b>Address: <code>{}</code></b>\n\
        <b>Private Key: <code>{}</code></b>\n\n",
        wallets_addresses[0],
        wallets_pks[0].to_hex_string(),
        wallets_addresses[1],
        wallets_pks[1].to_hex_string(),
        wallets_addresses[2],
        wallets_pks[2].to_hex_string(),
    );

    let keyboard = InlineKeyboardMarkup::new(vec![vec![get_close_button()]]);

    Some((text, keyboard))
}
pub async fn check_password_before_menu(
    func: fn(&Bot, i64) -> Option<(String, InlineKeyboardMarkup)>,
    bot: &Bot,
    user: &User,
    reply_action: ReplyAction,
) -> Option<(String, InlineKeyboardMarkup)> {
    let user_id_number = user.id.0 as i64;
    if is_passwd_set(user_id_number) {
        return {
            ask_for_password(bot, user, reply_action).await;
            None
        };
    };
    func(bot, user_id_number)
}

pub async fn ask_for_password(
    bot: &Bot,
    user: &User,
    reply_action: ReplyAction,
) -> Option<(String, InlineKeyboardMarkup)> {
    let msg_to_send = "Asking for password";
    let msg_reply = "Your password";

    send_message_force_reply_update_action(bot, user, msg_to_send, msg_reply, reply_action).await;
    None
}
