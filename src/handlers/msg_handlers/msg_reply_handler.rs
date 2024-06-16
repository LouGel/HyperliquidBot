use super::reply_actions::*;
use crate::bot::*;
use crate::handlers::constants_callbacks::*;
use crate::handlers::handle_send_tx_action;
use crate::menus::*;
use crate::traits::*;
use crate::types::*;
use crate::utils::*;
use anyhow::anyhow;
use ethers_core::k256::elliptic_curve::SecretKey;
use orders_to_make_menu::spawn_order_menu_from_keyboard;
use teloxide::prelude::*;
use teloxide_core::types::*;

pub fn get_text_from_msg(msg: &Message) -> anyhow::Result<String> {
    let text = msg
        .text()
        .ok_or(anyhow!("No text found for message {:?}", msg))?;
    Ok(text.to_owned())
}
pub fn message_handler_reply(
    bot: &Bot,
    msg: &Message,
    user_from: &User,
    resp_action: Action,
) -> anyhow::Result<()> {
    let user_id_number = user_from.id.0 as i64;
    let bot = bot.clone();
    let user_from = user_from.clone();

    let msg_text = get_text_from_msg(&msg)?;

    match resp_action.reply_action {
        ReplyAction::ImportWallet(iw_struct) => {
            handle_import_wallet_action(&bot, &user_from, iw_struct, msg_text)?;
        }

        ReplyAction::ShowPk => {
            if !is_correct_password(user_id_number, msg_text.clone()) {
                send_error(&bot, &user_from, "Invalid password retry the process");
            } else {
                if let Some((text, keyboard)) = pks(&bot, user_id_number) {
                    send_message_with_buttons(&bot, &user_from, &text, &keyboard);
                }
            }
            close_reply_action(user_id_number);
        }

        ReplyAction::ReplaceWallet(pk_no) => {
            if is_correct_password(user_id_number, msg_text.clone()) {
                let p_key = SecretKey::generate_random().unwrap();
                tokio::spawn(async move {
                    change_wallet(&bot.clone(), p_key, &user_from, pk_no).await;
                });
            } else {
                send_error(&bot, &user_from, "Invalid password retry the process");
            }
            close_reply_action(user_id_number);
            close_reply_action(user_id_number);
        }

        ReplyAction::SetPasswd(set_psswd_struct) => {
            tokio::spawn(async move {
                handle_set_passwd_action(
                    &bot,
                    &user_from,
                    user_id_number,
                    set_psswd_struct,
                    msg_text,
                )
                .await
            });
        }

        ReplyAction::CancelOrder(step, message_from) => match step.clone() {
            CancelOrderStep::AskForOrderNo => {
                let mut order_no = msg_text.clone();
                // if let Some(wallet_index) = is_a_kyber_swap_order(&message_from.text, &mut order_no)
                // {
                let new_action = ReplyAction::CancelOrder(
                    CancelOrderStep::AnswerOrderNo(OrderNo { no: order_no }),
                    message_from,
                );

                if !is_passwd_set(user_id_number) {
                    tokio::spawn(async move {
                        handle_send_tx_action(new_action, &bot, user_from).await
                    });
                } else {
                    tokio::spawn(
                        async move { ask_for_password(&bot, &user_from, new_action).await },
                    );
                }
                // } else {
                //     send_error(&bot, &user_from, "Invalid order id");
                //     close_reply_action(user_id_number);
                //     return Ok(());
                // }
            }
            CancelOrderStep::AnswerOrderNo(_) => {
                if is_correct_password(user_id_number, msg_text.clone()) {
                    let reply_action_clone = ReplyAction::CancelOrder(step, message_from);
                    tokio::spawn(async move {
                        handle_send_tx_action(reply_action_clone, &bot, user_from).await
                    });
                } else {
                    send_error(&bot, &user_from, "Invalid password retry the process");
                }
            }
        },
        ReplyAction::SetAmountPlain(callback, message_from) => handle_custom_setters(
            &bot,
            &user_from,
            &callback,
            &mut msg_text.replace(",", "."),
            message_from,
            is_good_amount_format,
            "Wrong amount",
        ),
        ReplyAction::SetTokenName(message_from) => handle_custom_setters(
            &bot,
            &user_from,
            TOKEN_NAME,
            &mut msg_text.clone(),
            message_from,
            is_good_token_name_or_number,
            "Wrong token name",
        ),

        x => {
            if is_correct_password(user_id_number, msg_text.clone()) {
                tokio::spawn(async move { handle_send_tx_action(x, &bot, user_from).await });
            } else {
                send_error(&bot, &user_from, "Invalid password retry the process");
            }
            close_reply_action(user_id_number);
        }
    }
    Ok(())
}
fn handle_custom_setters(
    bot: &Bot,
    user: &User,
    callback_to_find: &str,
    input: &mut String,
    mut message_to_reply: MessageToReply,
    verifier: fn(&str, &mut String) -> bool,
    error_message: &str,
) {
    let msg_id = MessageId(message_to_reply.id);
    if verifier(&message_to_reply.text, input) {
        message_to_reply
            .keyboard
            .change_text_where_callback_contains(callback_to_find, input);
        if callback_to_find.contains(TOKEN_NAME) {
            let bot = bot.clone();
            let user = user.clone();
            tokio::spawn(async move {
                // &format!("{desired_token} ({price_usd}$) ✏️");
                spawn_order_menu_from_keyboard(&bot, &user, msg_id, message_to_reply.keyboard).await
            });
            return;
        }

        debug!(
            "Message keyboard -> {:?}, callback: {}",
            message_to_reply.keyboard, callback_to_find
        );
        modify_message_with_buttons(
            bot,
            user,
            msg_id,
            &message_to_reply.text,
            &message_to_reply.keyboard,
        )
    } else {
        send_error(&bot, &user, error_message);
    }
    close_reply_action(user.id.0 as i64);
}

// ReplyAction::SetDuration(message_from) => handle_custom_setters(
//     &bot,
//     &user_from,
//     SET_DURATION,
//     &mut msg_text.clone(),
//     message_from,
//     is_good_duration_format,
//     "Wrong duration format",
// ),

// ReplyAction::SetAddress(callback, message_from) => handle_custom_setters(
//     &bot,
//     &user_from,
//     &callback,
//     &mut msg_text.replace(" ", ""),
//     message_from,
//     is_ethereum_public_key,
//     "Wrong evm pub key format",
// ),

// ReplyAction::SetAmountPerc(callback, message_from) => handle_custom_setters(
//     &bot,
//     &user_from,
//     &callback,
//     &mut msg_text.replace(",", "."),
//     message_from,
//     is_good_percent_format,
//     "Wrong percentage format",
// ),
