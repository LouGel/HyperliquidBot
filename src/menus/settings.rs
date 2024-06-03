use crate::get_main_and_faq_banner;

use crate::handlers::constants_callbacks::*;
use teloxide::types::{InlineKeyboardButton, InlineKeyboardMarkup};

pub async fn settings_menu() -> anyhow::Result<(String, InlineKeyboardMarkup)> {
    let text =
        "<b>Settings\n⚙️ Manage various wallet settings on your Telegram account.</b>".to_owned();

    let keyboard = get_settings_keyboard();

    Ok((text, keyboard))
}

pub fn get_settings_keyboard() -> InlineKeyboardMarkup {
    InlineKeyboardMarkup::new(vec![
        get_main_and_faq_banner(),
        vec![
            InlineKeyboardButton::callback(
                "Replace Wallet",
                &format!("{SIMPLE_MENU}_{REPLACE_MENU}"),
            ),
            InlineKeyboardButton::callback(
                "Import Wallet",
                &format!("{SIMPLE_MENU}_{IMPORT_MENU}"),
            ),
            InlineKeyboardButton::callback("Private Keys", &format!("{REPLY_ACT}_{SHOW_PK}")),
        ],
        vec![InlineKeyboardButton::callback(
            "Set Password",
            &format!("{REPLY_ACT}_{SET_PASSWD}"),
        )],
        // vec![InlineKeyboardButton::callback(
        //     "Create Referral",
        //    &format!("{SIMPLE_MENU}_SetRef",
        // )],
    ])
}
