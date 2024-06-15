use crate::globals::DEAD_CALLBACK;
use crate::handlers::constants_callbacks::*;

use teloxide::prelude::*;

use crate::handlers::{
    dynamic_menus_handler, msg_handlers::reply_action_handler, simple_action_handler,
    simple_menus_handler,
};
use crate::send_unexpected_callback_function_error;
use crate::{send_unexpected_error, types::*};

pub async fn callback_handler(bot: Bot, q: CallbackQuery) -> anyhow::Result<()> {
    if let Some(callback_function) = q.data.as_deref() {
        debug!("Callback : {} activates", callback_function);
        if !callback_function.contains(DEAD_CALLBACK) {
            let opts: Vec<&str> = callback_function.split('_').collect();

            let user = q.from.to_owned();
            if opts.len() > 1 {
                bot.answer_callback_query(&q.id).await?;

                let msg_from = q
                    .clone()
                    .message
                    .ok_or(anyhow::anyhow!("No msg from {:?}", q))?;

                match opts[0] {
                    SIMPLE_MENU => simple_menus_handler(&bot, user, opts, msg_from).await,

                    REFRESH_MENU => refresh_menu(&bot, user, opts, msg_from).await,
                    REPLY_ACT => {
                        if let Ok(reply_action) =
                            ReplyAction::from_str(opts[1], &msg_from, opts.last().unwrap())
                        {
                            reply_action_handler(&bot, user, reply_action).await;
                        } else if let Ok(reply_action) =
                            ReplyAction::from_str(opts[2], &msg_from, opts.last().unwrap())
                        {
                            reply_action_handler(&bot, user, reply_action).await;
                        } else {
                            send_unexpected_error(
                                &bot,
                                &user,
                                format!("Callback function {} not executable", callback_function),
                            )
                        }
                    }
                    DYN_ACTION => {
                        if let Err(e) = dynamic_menus_handler(
                            &bot,
                            user.clone(),
                            opts,
                            msg_from,
                            callback_function,
                        )
                        .await
                        {
                            send_unexpected_error(
                                &bot,
                                &user,
                                format!("Dynmenu : {}", e.to_string()),
                            )
                        }
                    }
                    SIMPLE_ACTION => simple_action_handler(&bot, user, opts, msg_from).await,
                    _ => send_unexpected_callback_function_error(&bot, &user, callback_function),
                }
            } else {
                send_unexpected_callback_function_error(&bot, &user, callback_function)
            }
        }
    }
    Ok(())
}

use teloxide::types::User;

pub async fn refresh_menu(bot: &Bot, user: User, menu: Vec<&str>, msg: Message) {
    if let Some(keyboard) = msg.reply_markup() {
        // match menu[1] {
        //     BUY_MENU => spawn_buy_menu_from_keyboard(bot, &user, msg.id, keyboard.to_owned()).await,
        //     SELL_MENU => {
        //         spawn_sell_menu_from_keyboard(bot, &user, msg.id, keyboard.to_owned()).await
        //     }
        //     SELL_LIMIT_MENU => {
        //         spawn_limit_sell_menu_from_keyboard(bot, &user, msg.id, keyboard.to_owned()).await
        //     }
        //     BUY_LIMIT_MENU => {
        //         spawn_order_menu_from_keyboard(bot, &user, msg.id, keyboard.to_owned()).await
        //     }
        //     _ => send_unexpected_callback_function_error(&bot, &user, &menu.join("_")),
        // }
    } else {
        send_unexpected_error(
            bot,
            &user,
            format!("No reply_markup in message {}", menu.join("_")),
        )
    }
}
