use super::*;
use crate::ask_for_password;
use crate::bot::*;
use crate::types::*;
use crate::utils::*;

use teloxide::prelude::*;
use teloxide_core::types::*;
pub async fn handle_set_passwd_action(
    bot: &Bot,
    user_from: &User,
    user_id_number: i64,
    mut set_psswd_struct: SetPasswd,
    msg_text: String,
) -> anyhow::Result<()> {
    if set_psswd_struct.passwd_given_twice {
        if is_correct_password(user_id_number, msg_text) {
            let _ = set_passwd(user_id_number, set_psswd_struct.first_password.unwrap()).await;
            send_message(bot, user_from, "Password set");
        } else {
            send_error_and_close(bot, user_from, "Invalid password");
        }
    } else if let Some(first_passw) = &set_psswd_struct.first_password {
        if *first_passw == msg_text {
            if is_passwd_set(user_id_number) {
                set_psswd_struct.passwd_given_twice = true;
                ask_for_password(bot, user_from, ReplyAction::SetPasswd(set_psswd_struct)).await;
            } else {
                let _ = set_passwd(user_id_number, msg_text).await;
                send_message(bot, user_from, "Password set");
            }
        } else {
            send_error_and_close(bot, user_from, "Password doesn't match");
        }
    } else {
        set_psswd_struct.first_password = Some(msg_text);

        send_message_force_reply_update_action(
            &bot,
            &user_from,
            "Repeat your passwd",
            "repeating password",
            ReplyAction::SetPasswd(set_psswd_struct),
        )
        .await;
    }

    Ok(())
}
