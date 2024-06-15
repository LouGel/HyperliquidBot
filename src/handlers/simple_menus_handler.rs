use super::constants_callbacks::*;
use super::get_reply_markup_from_msg;
use crate::bot::*;
use crate::menus::*;
use crate::orders_to_make_menu::make_orders_menu;
use teloxide::prelude::*;
use teloxide::types::User;

pub async fn simple_menus_handler(bot: &Bot, user: User, menu: Vec<&str>, msg: Message) {
    let menu_result = match menu[1] {
        MAIN_MENU => main_menu(user.id).await,
        TRADE_MENU => trade_menu().await,
        SETTINGS_MENU => settings_menu().await,
        REPLACE_MENU => replace_wallet_menu(),
        IMPORT_MENU => import_wallet_menu(),
        // BUY_MENU => buy_menu(user.id, None).await,
        // SELL_MENU => sell_menu(&user).await,
        MANAGE_ORDERS_MENU => orders_menu(&user).await,
        MAKE_ORDERS_MENU => {
            if menu.len() > 3 {
                make_orders_menu(user.id, "WAGMI", menu[2] == BUY, menu[3] == LIMIT).await
            } else {
                send_unexpected_callback_function_error(bot, &user, &menu.join("_"));
                return;
            }
        }

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
