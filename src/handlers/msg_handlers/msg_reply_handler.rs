use std::str::FromStr;

use anyhow::anyhow;
use ethers::abi::Address;
use ethers_core::k256::elliptic_curve::SecretKey;

use super::reply_actions::*;
use crate::address;
use crate::bot::*;
use crate::get_pool;
use crate::handlers::constants_callbacks::*;
use crate::handlers::handle_send_tx_action;
use crate::menus::*;
use crate::traits::*;
use crate::types::*;
use crate::utils::*;
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
        ReplyAction::SetAddress(callback, message_from) => handle_custom_setters(
            &bot,
            &user_from,
            &callback,
            &mut msg_text.replace(" ", ""),
            message_from,
            is_ethereum_public_key,
            "Wrong evm pub key format",
        ),

        ReplyAction::SetSlippage(message_from) => handle_custom_setters(
            &bot,
            &user_from,
            SLIPPAGE,
            &mut msg_text.replace(",", "."),
            message_from,
            is_good_percent_format,
            "Wrong slippage",
        ),
        ReplyAction::SetAmountPerc(callback, message_from) => handle_custom_setters(
            &bot,
            &user_from,
            &callback,
            &mut msg_text.replace(",", "."),
            message_from,
            is_good_percent_format,
            "Wrong percentage format",
        ),
        ReplyAction::CancelOrder(step, message_from) => match step.clone() {
            CancelOrderStep::AskForOrderNo => {
                let mut order_no = msg_text.clone();
                if let Some(wallet_index) = is_a_kyber_swap_order(&message_from.text, &mut order_no)
                {
                    let new_action = ReplyAction::CancelOrder(
                        CancelOrderStep::AnswerOrderNo(OrderNo {
                            wallet_index,
                            no: order_no,
                        }),
                        message_from,
                    );

                    if !is_passwd_set(user_id_number) {
                        tokio::spawn(async move {
                            handle_send_tx_action(new_action, &bot, user_from).await
                        });
                    } else {
                        tokio::spawn(async move {
                            ask_for_password(&bot, &user_from, new_action).await
                        });
                    }
                } else {
                    send_error(&bot, &user_from, "Invalid order id");
                    close_reply_action(user_id_number);
                    return Ok(());
                }
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
            SET_TOKEN_NAME,
            &mut msg_text.clone(),
            message_from,
            is_good_token_name_or_number,
            "Wrong token name",
        ),
        ReplyAction::SetDuration(message_from) => handle_custom_setters(
            &bot,
            &user_from,
            SET_DURATION,
            &mut msg_text.clone(),
            message_from,
            is_good_duration_format,
            "Wrong duration format",
        ),
        ReplyAction::SetNegativePerc(message_from) => handle_custom_setters(
            &bot,
            &user_from,
            SET_NEGATIVE_PERC,
            &mut msg_text.replace(",", "."),
            message_from,
            is_good_neg_percent_format,
            "Wrong \\-% format",
        ),
        ReplyAction::SetPositivePerc(message_from) => handle_custom_setters(
            &bot,
            &user_from,
            SET_POSITIVE_PERC,
            &mut msg_text.replace(",", "."),
            message_from,
            is_good_pos_percent_format,
            "Wrong \\+% format",
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
    debug!("In handle reply : {}", &callback_to_find);
    if verifier(&message_to_reply.text, input) {
        if !callback_to_find.contains("Set") {
            message_to_reply
                .keyboard
                .update_custom_fct(callback_to_find.to_string(), input);
        } else {
            debug!("Changing message reply");
            message_to_reply
                .keyboard
                .change_text_where_callback_contains(callback_to_find, &input);

            if callback_to_find.contains(SET_ADDRESS) {
                let bot = bot.clone();
                let user = user.clone();
                tokio::spawn(async move {
                    let _ = match message_to_reply.text.contains("imit") {
                        true => {
                            spawn_limit_buy_menu_from_keyboard(
                                &bot,
                                &user,
                                msg_id,
                                message_to_reply.keyboard,
                            )
                            .await
                        }
                        false => {
                            spawn_buy_menu_from_keyboard(
                                &bot,
                                &user,
                                msg_id,
                                message_to_reply.keyboard,
                            )
                            .await
                        }
                    };
                });
                return;
            } else if callback_to_find.contains(SET_TOKEN_NAME) {
                let bot = bot.clone();
                let user = user.clone();
                tokio::spawn(async move {
                    let _ = match message_to_reply.text.contains("imit") {
                        true => {
                            spawn_limit_sell_menu_from_keyboard(
                                &bot,
                                &user,
                                msg_id,
                                message_to_reply.keyboard,
                            )
                            .await
                        }
                        false => {
                            spawn_sell_menu_from_keyboard(
                                &bot,
                                &user,
                                msg_id,
                                message_to_reply.keyboard,
                            )
                            .await
                        }
                    };
                });
                return;
            } else if callback_to_find.contains(SET_TOKEN_DB) {
                let bot = bot.clone();
                let user = user.clone();
                let input_addr = address!(input);
                let vec_keyboard = message_to_reply.keyboard.inline_keyboard;
                tokio::spawn(async move {
                    check_and_add_token_to_db(&bot, &user, input_addr, vec_keyboard).await;
                });

                return;
            }
        }
        debug!("Not returning");

        modify_buttons(&bot, user.id, msg_id, message_to_reply.keyboard)
    } else {
        send_error(&bot, &user, error_message);
    }
    close_reply_action(user.id.0 as i64);
}

pub async fn check_and_add_token_to_db(
    bot: &Bot,
    user: &User,
    address: Address,
    keyboard: Vec<Vec<InlineKeyboardButton>>,
) {
    let pool = get_pool();
    let chain = keyboard[2][0].text.to_lowercase();
    debug!("In check and add token to db");

    todo!("check_and_add_token_to_db")
    // match pool.push_token(&network, address).await {
    //     Ok(symbol) => send_message(bot, user, &format!("{} added tyo your tokens", symbol)),
    //     Err(e) => {
    //         let err_str = e.to_string();
    //         if err_str.contains("already") {
    //             send_error(bot, user, &err_str)
    //         } else {
    //             send_unexpected_error(bot, user, err_str);
    //         }
    //     }
    // }
}
