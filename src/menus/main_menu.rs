use crate::format::format_float;
use crate::globals::*;
use crate::hyperliquid_api::*;
use crate::utils::keys_and_addresses::*;
use ethers::utils::format_units;
use teloxide::types::{InlineKeyboardButton, InlineKeyboardMarkup, UserId};
use tokio::join;
pub async fn main_menu(user_id: UserId) -> anyhow::Result<(String, InlineKeyboardMarkup)> {
    let user_pks = WALLETS_PKEY.get_result(user_id)?;
    let user_addresses = vec_3_p_keys_to_address(&user_pks);
    let gas_price = "123".to_owned();
    let balances_raw = Vec::new();

    let text = format_text_main_menu(gas_price, user_addresses, balances_raw)?;

    let inline_keyboard = get_main_menu_keyboard();

    Ok((text, inline_keyboard))
}
use ethers::types::{Address, U256};
fn format_text_main_menu(
    gas_price: String,
    addresses: Vec<Address>,
    balances_raw: Vec<U256>,
) -> anyhow::Result<String> {
    let mut text = format!(
        "<b>🤖 Hyperliquid X - Your Ultimate Crypto Companion\n\n\
        ═══ Your Wallets ═══</b> \n\n\
        "
    );
    for (index, x) in balances_raw.into_iter().enumerate() {
        text += &format!(
            "<b>👝 Wallet⬩w{}</b>\n\
    <b>Balance: <code>{} USD</code> ⬩</b>\n\
    <b>Address: <code>{}</code></b>\n\n\"",
            index + 1,
            x,
            addresses[index].to_full_string()
        )
    }

    Ok(text)
}
use crate::handlers::constants_callbacks::*;

pub fn get_main_menu_keyboard() -> InlineKeyboardMarkup {
    InlineKeyboardMarkup::new([[
        InlineKeyboardButton::callback("🎯 Snipe", &format!("{SIMPLE_MENU}_{SNIPE_MENU}")),
        InlineKeyboardButton::callback("💼 Trade", &format!("{SIMPLE_MENU}_{TRADE_MENU}")),
    ]])
}
