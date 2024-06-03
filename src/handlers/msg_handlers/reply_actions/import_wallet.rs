use super::*;
use crate::menus::*;
use crate::types::*;
use crate::utils::*;
use ethers_core::k256::elliptic_curve::SecretKey;
use ethers_core::k256::Secp256k1;
use teloxide::prelude::*;
use teloxide_core::types::*;

pub fn handle_import_wallet_action(
    bot: &Bot,
    user_from: &User,
    iw_struct: ImportWallet,
    msg_text: String,
) -> anyhow::Result<()> {
    let user_id_no = user_from.id.0 as i64;
    if let Some(p_key_str) = iw_struct.clone().private_key {
        process_existing_private_key(bot, user_from, user_id_no, &iw_struct, &msg_text, p_key_str)
    } else {
        process_new_private_key(bot, user_from, user_id_no, iw_struct, &msg_text)
    }
}

fn process_existing_private_key(
    bot: &Bot,
    user_from: &User,
    user_id_no: i64,
    iw_struct: &ImportWallet,
    msg_text: &str,
    p_key_str: String,
) -> anyhow::Result<()> {
    if is_correct_password(user_id_no, msg_text.to_string()) {
        let bot = bot.clone();
        let user_from = user_from.clone();
        let wallet_no = iw_struct.no;
        let pk_raw = hex::decode(&p_key_str).expect("Failed to decode hex string");
        let private_key: SecretKey<Secp256k1> =
            SecretKey::from_slice(&pk_raw).expect("Failed to create SecretKey from slice");

        tokio::spawn(async move {
            change_wallet(&bot, private_key, &user_from, wallet_no).await;
        });
        close_reply_action(user_id_no);
    } else {
        send_error_and_close(bot, user_from, "Invalid password")
    }
    Ok(())
}

fn process_new_private_key(
    bot: &Bot,
    user_from: &User,
    user_id_no: i64,
    iw_struct: ImportWallet,
    msg_text: &str,
) -> anyhow::Result<()> {
    let bot = bot.clone();
    let user_from = user_from.clone();
    let mut iw_struct = iw_struct.clone();
    if is_ethereum_private_key(msg_text) {
        if !is_passwd_set(user_id_no) {
            let pk_raw = hex::decode(msg_text).expect("Failed to decode hex string");
            let private_key: SecretKey<Secp256k1> =
                SecretKey::from_slice(&pk_raw).expect("Failed to create SecretKey from slice");
            tokio::spawn(async move {
                change_wallet(&bot, private_key, &user_from, iw_struct.no).await;
            });
            close_reply_action(user_id_no);
        } else {
            iw_struct.private_key = Some(msg_text.to_string());
            tokio::spawn(async move {
                ask_for_password(
                    &bot,
                    &user_from,
                    ReplyAction::ImportWallet(iw_struct.clone()),
                )
                .await;
            });
        }
    } else {
        send_error_and_close(&bot, &user_from, "Invalid pk")
    }
    Ok(())
}
