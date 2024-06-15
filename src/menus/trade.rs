use crate::get_main_and_faq_banner;
use teloxide::types::{InlineKeyboardButton, InlineKeyboardMarkup};

pub async fn trade_menu() -> anyhow::Result<(String, InlineKeyboardMarkup)> {
    let text = format!(
        "<b>💹WAGMI TRADING</b>
                Trade tokens on Hyperliquid so that we’re all gonna make it!\n\n\
                Choose the trading method that suits you best\n\
                → Buy / Sell Tokens (buy directly any Token on chains)\n\
                → Buy / Sell Limit (buy any Token on chains)\n"
    );

    let inline_keyboard = get_trade_keyboard();
    Ok((text, inline_keyboard))
}
use crate::handlers::constants_callbacks::*;

pub fn get_trade_keyboard() -> InlineKeyboardMarkup {
    InlineKeyboardMarkup::new(vec![
        get_main_and_faq_banner(),
        vec![
            InlineKeyboardButton::callback(
                "Buy Tokens",
                &format!("{SIMPLE_MENU}_{MAKE_ORDERS_MENU}_{BUY}_{MARKET}"),
            ),
            InlineKeyboardButton::callback(
                "Sell Tokens",
                &format!("{SIMPLE_MENU}_{MAKE_ORDERS_MENU}_{SELL}_{MARKET}"),
            ),
        ],
        vec![
            InlineKeyboardButton::callback(
                "Buy Limit",
                &format!("{SIMPLE_MENU}_{MAKE_ORDERS_MENU}_{BUY}_{LIMIT}"),
            ),
            InlineKeyboardButton::callback(
                "Sell Limit",
                &format!("{SIMPLE_MENU}_{MAKE_ORDERS_MENU}_{SELL}_{LIMIT}"),
            ),
        ],
        vec![
            InlineKeyboardButton::callback(
                "Current orders",
                &format!("{SIMPLE_MENU}_{MANAGE_ORDERS_MENU}"),
            ),
            // InlineKeyboardButton::callback("Transfer", &format!("{SIMPLE_MENU}_{TRANSFER_MENU}")),
            InlineKeyboardButton::callback("Balances", &format!("{SIMPLE_MENU}_{BALANCES_MENU}")),
        ],
    ])
}
