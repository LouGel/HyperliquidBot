use ethers_core::k256::elliptic_curve::SecretKey;
use teloxide::prelude::*;
use teloxide_core::types::*;

use super::reply_actions::*;
use crate::bot::*;
use crate::handlers::ResultHandler;
use crate::hyperliquid_api::orders::cancel_order;
use crate::hyperliquid_api::{buy_from_menu, order_from_menu, sell_from_menu};
use crate::menus::*;
use crate::traits::*;
use crate::types::*;
use crate::utils::*;

pub async fn reply_action_handler(bot: &Bot, user: User, reply_action: ReplyAction) {
    let user_id_no = user.id.0 as i64;
    let user_clone = user.clone();
    if let Some((msg_tosend, msg_reply)) = match reply_action.clone() {
        ReplyAction::ShowPk => {
            if let Some((text, keyboard)) =
                check_password_before_menu(pks, bot, &user, reply_action.clone()).await
            {
                send_message_with_buttons(bot, &user_clone, &text, &keyboard)
            }

            None
        }
        ReplyAction::ReplaceWallet(pk_no) => {
            if is_passwd_set(user_id_no) {
                ask_for_password(bot, &user, reply_action.clone()).await;
            } else {
                let p_key = SecretKey::generate_random().unwrap();
                let bot = bot.clone();
                tokio::spawn(async move {
                    change_wallet(&bot, p_key, &user, pk_no).await;
                });
            }
            None
        }

        ReplyAction::ImportWallet(_) => {
            Some(("Importing private key", "Please write a private key"))
        }
        ReplyAction::SetPasswd(_) => Some(("Setting new password", "Please givew a new password")),
        ReplyAction::SetAddress(..) => Some(("Setting new address", "Please give an address")),
        ReplyAction::SetAmountPlain(..) => Some(("Setting Amount", "Please give an amount")),
        ReplyAction::SetTokenName(_) => Some(("Setting token ", "Enter the name/no of the token")),
        ReplyAction::SetAmountPerc(..) => {
            Some(("Setting Amount in % ", "Please give an amount in % "))
        }
        ReplyAction::SetSlippage(_) => Some(("Setting Slippage", "Please give a slippage")),
        ReplyAction::SetDuration(_) => Some(("Setting Duration", "Number \\+ h/d/y")),
        ReplyAction::SetPositivePerc(_) => Some(("Setting Limit Sell \\+%", "Percentage")),
        ReplyAction::SetNegativePerc(_) => Some(("Setting Limit Buy \\-%", "Percentage")),
        ReplyAction::CancelOrder(..) => Some(("Enter the order id to cancel", "Order Id")),
        _ => {
            if is_passwd_set(user_id_no) {
                ask_for_password(bot, &user, reply_action.clone()).await;
            } else {
                handle_send_tx_action(reply_action.clone(), bot, user).await;
            }
            None
        }
    } {
        send_message_force_reply_update_action(
            bot,
            &user_clone,
            msg_tosend,
            msg_reply,
            reply_action,
        )
        .await;
    }
}

pub async fn handle_send_tx_action(action: ReplyAction, bot: &Bot, user: User) {
    match action.clone() {
        ReplyAction::CancelOrder(step, _) => {
            // match cancel_order(user.id, action).await {
            //     Ok(()) => send_message(bot, &user, "Order cancelled successfully"),
            //     Err(e) => send_unexpected_error(
            //         bot,
            //         &user,
            //         format!("Error canceling order {:?} -> {}", step, e.to_string()),
            //     ),
            // };
        }

        ReplyAction::Buy(msg_info) => {
            if let Err(e) = order_from_menu(bot, &user, msg_info.keyboard).await {
                send_error(
                    bot,
                    &user,
                    &("Error in buy limit from menu".to_owned() + &e.to_string()),
                )
            } else {
                send_message(bot, &user, "Order Created")
            }
        }
        ReplyAction::BuyLimit(msg_info) => {
            if let Err(e) = order_from_menu(bot, &user, msg_info.keyboard).await {
                send_error(
                    bot,
                    &user,
                    &("Error in buy limit from menu".to_owned() + &e.to_string()),
                )
            } else {
                send_message(bot, &user, "Order Created")
            }
        }
        ReplyAction::SellLimit(msg_info) => {
            if let Err(e) = sell_from_limit_sell_menu(&user, msg_info.keyboard).await {
                send_error(
                    bot,
                    &user,
                    &("Error in Sell limit from menu".to_owned() + &e.to_string()),
                )
            } else {
                send_message(bot, &user, "Order Created")
            }
        }
        x => send_unexpected_error(
            bot,
            &user,
            format!("handle_send_tx_action :{:?} -> not expected", x),
        ),
    }
}
