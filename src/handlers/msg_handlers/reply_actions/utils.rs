use crate::bot::*;
use crate::globals::*;
use crate::traits::*;
use ethers_core::k256::elliptic_curve::SecretKey;
use ethers_core::k256::Secp256k1;

use teloxide::prelude::*;

use teloxide_core::types::*;

pub fn close_reply_action(user_id: i64) {
    let mut action_map = REPLY_ACTION.lock().unwrap();
    action_map.remove(&user_id);
}
pub fn send_error_and_close(bot: &Bot, user: &User, msg_to_send: &str) {
    send_error(bot, &user, msg_to_send);
    close_reply_action(user.id.0 as i64)
}

pub async fn change_wallet(bot: &Bot, p_key: SecretKey<Secp256k1>, user: &User, pk_no: u8) {
    let user_id_number = user.id.0 as i64;
    let pool = get_pool();

    if let Ok(()) = pool.push_one_pks(user_id_number, p_key, pk_no).await {
        send_message(&bot, &user, &format!("Private key replaced in w{pk_no}"));
    } else {
        send_error(&bot, &user, "Error in pushing , please retry the process");
    }
}
