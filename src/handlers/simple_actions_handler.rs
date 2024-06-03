use crate::{bot::*, InlineKeyBoardHandler};
use url::Url;

use anyhow::anyhow;
use teloxide::prelude::*;
use teloxide_core::types::InlineKeyboardMarkup;

use crate::globals::*;
use crate::handlers::constants_callbacks::*;
use crate::menus::main_menu;
use crate::traits::{OmnixString, PoolOperation};
use teloxide::types::User;

pub fn get_reply_markup_from_msg(msg: &Message) -> anyhow::Result<InlineKeyboardMarkup> {
    let markup = msg
        .reply_markup()
        .ok_or(anyhow!("No markup for this message {:?}", msg))?;
    Ok(markup.to_owned())
}
pub async fn simple_action_handler(bot: &Bot, user: User, menu: Vec<&str>, msg: Message) {
    let user_id_number = user.id.0 as i64;
    let pool = get_pool();
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

pub trait ResultHandler {
    fn handle_action_result(self, prefix: &str, bot: &Bot, user: &User);
    fn handle_error(err: anyhow::Error, prefix: &str, bot: &Bot, user: &User);
}

impl ResultHandler for anyhow::Result<Url> {
    fn handle_action_result(self, prefix: &str, bot: &Bot, user: &User) {
        match self {
            Ok(url) => {
                let message = format!("{}Tx succeeded: {}", prefix, url);
                debug!("Prefix : {} \n\n Url {}\n\n Total {}", prefix, url, message);
                send_message(&bot, &user, &message);
            }
            Err(e) => Self::handle_error(e, prefix, bot, user),
        }
    }

    fn handle_error(err: anyhow::Error, prefix: &str, bot: &Bot, user: &User) {
        let error_message = err.to_string();

        if error_message.contains("0x08c379a") {
            match error_message.extract_hex_error_after_pattern("0x08c379a") {
                Ok(extracted_error) => {
                    if extracted_error.contains("Pausable") {
                        send_error(&bot, &user, &format!("{}: contract is paused", prefix));
                    } else if extracted_error.contains("ERC20") {
                        send_error(&bot, &user, &format!("{}: {}", prefix, extracted_error));
                    }
                }
                Err(_) => send_unexpected_error(&bot, &user, error_message),
            }
        } else if error_message.contains("exceeds balance") {
            send_error(
                &bot,
                &user,
                &format!(
                    "{}: You don't have enough tokens for the transaction",
                    prefix
                ),
            );
        } else if error_message.contains("insufficient funds") {
            send_error(
                &bot,
                &user,
                &format!("{}: Insufficient funds for transaction", prefix),
            );
        } else {
            send_unexpected_error(&bot, &user, error_message);
        }
    }
}
