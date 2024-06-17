use crate::globals::*;
use crate::types::hyperliquid_client::{Balance, HyperLiquidNetwork};
use crate::utils::keys_and_addresses::*;
use ethers::types::Address;
use teloxide::types::{InlineKeyboardButton, InlineKeyboardMarkup, UserId};
pub async fn main_menu(user_id: UserId) -> anyhow::Result<(String, InlineKeyboardMarkup)> {
    let user_pks = WALLETS_PKEY.get_result(user_id)?;
    let user_addresses = vec_3_p_keys_to_address(&user_pks);
    let client = HyperLiquidNetwork::get_client();
    let balances_raw = client
        .fetch_spot_balance_for_addresses(&user_addresses)
        .await?;
    let mut balances_usdc = Vec::new();
    for balances in &balances_raw {
        let usdc_balance: Vec<&Balance> = balances.iter().filter(|x| x.coin == "USDC").collect();
        balances_usdc.push(usdc_balance.into_iter().next())
    }
    let text = format_text_main_menu(user_addresses, balances_usdc)?;
    let inline_keyboard = get_main_menu_keyboard();

    Ok((text, inline_keyboard))
}

fn format_text_main_menu(
    addresses: Vec<Address>,
    balances_raw: Vec<Option<&Balance>>,
) -> anyhow::Result<String> {
    let mut text = format!(
        "<b>🤖 WAGMI TRADING BOT</b>
Buy, sell and interact with HyperLiquidX spot ecosystem anywhere, anytime.\n\n\
        <b>═══ Your Wallets ═══</b> \n\n\
        "
    );
    for (index, x) in balances_raw.into_iter().enumerate() {
        let bal = match x {
            Some(x) => x.total.clone(),
            None => "0".to_owned(),
        };
        text += &format!(
            "<b>💰 Wallet {}⬩w{}</b>\n\
    <b>Balance: <code>{} USDC</code> ⬩</b>\n\
    <b>Address: <code>{}</code></b>\n\n\"",
            index + 1,
            index + 1,
            bal,
            addresses[index].to_full_string()
        )
    }

    Ok(text.replace("\"", ""))
}
use crate::handlers::constants_callbacks::*;

pub fn get_main_menu_keyboard() -> InlineKeyboardMarkup {
    InlineKeyboardMarkup::new([[
        InlineKeyboardButton::callback("💼 Trade", &format!("{SIMPLE_MENU}_{TRADE_MENU}")),
        InlineKeyboardButton::callback("⚙️ Settings", &format!("{SIMPLE_MENU}_{SETTINGS_MENU}")),
    ]])
}
