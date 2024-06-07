use crate::get_main_and_faq_banner;
use teloxide::types::{InlineKeyboardButton, InlineKeyboardMarkup};

pub async fn trade_menu() -> anyhow::Result<(String, InlineKeyboardMarkup)> {
    let text = format!(
        "<b>💼 Trade anywhere at anytime.\n\n\
                Choose the trading method that suits you best\n\
                → Buy / Sell Tokens (buy directly any Token on chains)\n\
                → Buy / Sell Limit (buy any Token on chains)\n</b>"
    );

    let inline_keyboard = get_trade_keyboard();
    Ok((text, inline_keyboard))
}
use crate::handlers::constants_callbacks::*;

pub fn get_trade_keyboard() -> InlineKeyboardMarkup {
    InlineKeyboardMarkup::new(vec![
        get_main_and_faq_banner(),
        vec![
            InlineKeyboardButton::callback("Buy Tokens", &format!("{SIMPLE_MENU}_{BUY_MENU}")),
            InlineKeyboardButton::callback("Sell Tokens", &format!("{SIMPLE_MENU}_{SELL_MENU}")),
        ],
        vec![
            InlineKeyboardButton::callback("Buy Limit", &format!("{SIMPLE_MENU}_{BUY_LIMIT_MENU}")),
            InlineKeyboardButton::callback(
                "Sell Limit",
                &format!("{SIMPLE_MENU}_{SELL_LIMIT_MENU}"),
            ),
        ],
        vec![
            InlineKeyboardButton::callback(
                "Current orders",
                &format!("{SIMPLE_MENU}_{ORDERS_MENU}"),
            ),
            // InlineKeyboardButton::callback("Transfer", &format!("{SIMPLE_MENU}_{TRANSFER_MENU}")),
            InlineKeyboardButton::callback("Balances", &format!("{SIMPLE_MENU}_{BALANCES_MENU}")),
        ],
    ])
}
