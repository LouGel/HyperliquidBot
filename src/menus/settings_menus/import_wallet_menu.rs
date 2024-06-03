use crate::get_replace_import_keyboard;
use crate::handlers::constants_callbacks::IMPORT_WALLET;

use teloxide::types::InlineKeyboardMarkup;

pub fn import_wallet_menu() -> anyhow::Result<(String, InlineKeyboardMarkup)> {
    let text ="<b>🔧 Import Wallet - Replace an existing wallet by importing your wallet using a private key. Which wallet do you want to replace?</b>\n\
    <em> Wallet will be erased be sure you kept the private key </em>".to_string();

    let keyboard = get_replace_import_keyboard(IMPORT_WALLET);

    Ok((text, keyboard))
}
