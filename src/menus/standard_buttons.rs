use crate::globals::{DEAD_CALLBACK, WHITEPAPER_URL};
use crate::handlers::constants_callbacks::*;

use teloxide::types::{InlineKeyboardButton, InlineKeyboardMarkup};
use url::Url;

pub fn get_faq_button() -> InlineKeyboardButton {
    let whitepaper_url = Url::parse(WHITEPAPER_URL).unwrap();
    InlineKeyboardButton::url("❓ FAQ", whitepaper_url)
}
pub fn get_main_menu_button() -> InlineKeyboardButton {
    InlineKeyboardButton::callback("⏪ Main Menu", &format!("{SIMPLE_MENU}_{MAIN_MENU}"))
}

pub fn get_main_and_faq_banner() -> Vec<InlineKeyboardButton> {
    vec![get_main_menu_button(), get_faq_button()]
}

pub fn get_close_button() -> InlineKeyboardButton {
    InlineKeyboardButton::callback("✖️ Close", &format!("{SIMPLE_ACTION}_CLOSE"))
}

pub fn get_refresh_button(menu: &str) -> InlineKeyboardButton {
    InlineKeyboardButton::callback("🔄 Refresh Menu", &format!("{REFRESH_MENU}_{menu}"))
}
pub fn get_wallet_from_title_and_buttons(
    menu: &str,
) -> (InlineKeyboardButton, Vec<InlineKeyboardButton>) {
    (
        InlineKeyboardButton::callback("SELECT WALLET", DEAD_CALLBACK),
        vec![
            InlineKeyboardButton::callback("✅ W 1", &format!("{DYN_ACTION}_{menu}_{WALLET}_1")),
            InlineKeyboardButton::callback("W 2", &format!("{DYN_ACTION}_{menu}_{WALLET}_2")),
            InlineKeyboardButton::callback("W 3", &format!("{DYN_ACTION}_{menu}_{WALLET}_3")),
        ],
    )
}

// pub fn get_wallet_to_title_buttons_and_custom(
//     menu: &str,
// ) -> (
//     InlineKeyboardButton,
//     Vec<InlineKeyboardButton>,
//     InlineKeyboardButton,
// ) {
//     (
//         InlineKeyboardButton::callback("RECEIVER", DEAD_CALLBACK),
//         vec![
//             InlineKeyboardButton::callback("✅ W 1", &format!("{DYN_ACTION}_{menu}_{RECEIVER}_1")),
//             InlineKeyboardButton::callback("W 2", &format!("{DYN_ACTION}_{menu}_{RECEIVER}_2")),
//             InlineKeyboardButton::callback("W 3", &format!("{DYN_ACTION}_{menu}_{RECEIVER}_3")),
//         ],
//         InlineKeyboardButton::callback(
//             "Custom",
//             &format!("{REPLY_ACT}_{menu}_{RECEIVER}_{CUSTOM}"),
//         ),
//     )
// }

pub fn get_replace_import_keyboard(menu: &str) -> InlineKeyboardMarkup {
    InlineKeyboardMarkup::new(vec![
        vec![
            InlineKeyboardButton::callback("w1", &format!("{REPLY_ACT}_{menu}_1")),
            InlineKeyboardButton::callback("w2", &format!("{REPLY_ACT}_{menu}_2")),
            InlineKeyboardButton::callback("w3", &format!("{REPLY_ACT}_{menu}_3")),
        ],
        vec![get_close_button()],
    ])
}
// pub fn expert_keyboard_markup(menu_from: &str) -> Vec<Vec<InlineKeyboardButton>> {
//     vec![
//         vec![InlineKeyboardButton::callback("SLIPPAGE", DEAD_CALLBACK)],
//         vec![
//             InlineKeyboardButton::callback(
//                 "✅ 10%",
//                 &format!("{DYN_ACTION}_{menu_from}_{SLIPPAGE}_10"),
//             ),
//             InlineKeyboardButton::callback(
//                 "15%",
//                 &format!("{DYN_ACTION}_{menu_from}_{SLIPPAGE}_15"),
//             ),
//             InlineKeyboardButton::callback(
//                 "25%",
//                 &format!("{DYN_ACTION}_{menu_from}_{SLIPPAGE}_25"),
//             ),
//         ],
//         vec![
//             InlineKeyboardButton::callback(
//                 "50%",
//                 &format!("{DYN_ACTION}_{menu_from}_{SLIPPAGE}_50"),
//             ),
//             InlineKeyboardButton::callback(
//                 "75%",
//                 &format!("{DYN_ACTION}_{menu_from}_{SLIPPAGE}_75"),
//             ),
//             InlineKeyboardButton::callback("Custom", &format!("{REPLY_ACT}_{SLIPPAGE}_{CUSTOM}")),
//         ],
//     ]
// }
