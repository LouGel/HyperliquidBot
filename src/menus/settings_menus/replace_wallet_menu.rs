use crate::handlers::constants_callbacks::REPLACE_WALLET;
use teloxide::types::InlineKeyboardMarkup;

use crate::get_replace_import_keyboard;

pub fn replace_wallet_menu() -> anyhow::Result<(String, InlineKeyboardMarkup)> {
    let text = "<b>🔧 Import Wallet - Replace an existing wallet by a random generated one. Which wallet do you want to replace?</b>\n\
    <em> Wallet will be erased be sure you kept the private key </em>".to_string();

    let keyboard = get_replace_import_keyboard(REPLACE_WALLET);

    Ok((text, keyboard))
}
