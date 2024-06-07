use crate::bot::*;
use crate::menus::*;
use teloxide::prelude::*;
use teloxide::types::User;

use super::constants_callbacks::*;
use super::get_reply_markup_from_msg;

pub async fn simple_menus_handler(bot: &Bot, user: User, menu: Vec<&str>, msg: Message) {
    let menu_result = match menu[1] {
        MAIN_MENU => main_menu(user.id).await,
        TRADE_MENU => trade_menu().await,
        SETTINGS_MENU => settings_menu().await,
        REPLACE_MENU => replace_wallet_menu(),
        IMPORT_MENU => import_wallet_menu(),
        BUY_MENU => buy_menu(user.id, "-".to_string()).await,
        SELL_MENU => sell_menu(&user).await,
        ORDERS_MENU => orders_menu(&user).await,
        SELL_LIMIT_MENU => sell_limit_menu(&user).await,
        BUY_LIMIT_MENU => limit_buy_menu(user.id).await,
        BALANCES_MENU => balance_menu(&user).await,
        _ => {
            send_unexpected_callback_function_error(bot, &user, &menu.join("_"));
            return; // Consider a better way to handle this case, such as a default error response to the user
        }
    };

    if let Ok((text, keyboard)) = menu_result {
        match menu[0] {
            SIMPLE_MENU => send_message_with_buttons(&bot, &user, &text, &keyboard),
            UPDATE_CHAIN => modify_message_with_buttons(bot, &user, msg.id, &text, &keyboard),
            _ => send_unexpected_callback_function_error(bot, &user, &menu.join("_")),
        }
    } else if let Err(e) = menu_result {
        let e_str = &e.to_string();
        let cutted_err: Vec<&str> = e_str.split(':').collect();

        if cutted_err.len() > 1 && cutted_err[0].contains("Handled") {
            send_error(bot, &user, cutted_err[1])
        } else {
            send_unexpected_error(bot, &user, format!("Error in simplemenu :{:?}", e))
        }
    }
}
