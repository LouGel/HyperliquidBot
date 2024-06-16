use crate::types::hyperliquid_client::HyperLiquidNetwork;
use crate::{get_faq_button, get_main_menu_button};
use crate::{globals::*, vec_3_p_keys_to_address};
use anyhow::Result;
use ethers::types::Address;
use teloxide::{types::InlineKeyboardMarkup, types::User};

pub async fn balance_menu(user: &User) -> anyhow::Result<(String, InlineKeyboardMarkup)> {
    let pks = WALLETS_PKEY.get_result(user.id)?;
    let addresses = vec_3_p_keys_to_address(&pks);
    let mut text = format!(
        "<b>🤖 Hyperliquid 
    Your balances </b> \n"
    );

    text += &display_full_balance(addresses).await?;

    Ok((text, get_balance_keyboard()))
}

pub async fn display_full_balance(addresses: Vec<Address>) -> Result<String> {
    let client = HyperLiquidNetwork::get_client();
    let mut ret = String::new();
    let balances = client.fetch_spot_balance_for_addresses(&addresses).await?;
    for (i, wallet) in balances.iter().enumerate() {
        ret += &format!("\n<b>Wallet {i}-------\n</b>");
        let mut entered_loop = false;
        for balance in wallet.iter() {
            entered_loop = true;
            ret += &format!("{} : {} \n", balance.coin, balance.total)
        }
        if !entered_loop {
            ret += &format!("<i>Empty</i>");
        }
    }
    Ok(ret)
}
use crate::types::hyperliquid_client::Balance;
pub async fn display_token_balance(addresses: Vec<Address>, token: String) -> Result<String> {
    let client = HyperLiquidNetwork::get_client();
    let mut ret = format!("\n<b>Your {token} balance</b>\n");
    let balances_raw = client.fetch_spot_balance_for_addresses(&addresses).await?;
    for (i, balances) in balances_raw.iter().enumerate() {
        let usdc_balance: Vec<&Balance> = balances.iter().filter(|x| x.coin == token).collect();
        let bal = match usdc_balance.first() {
            Some(balance) => balance.total.clone(),
            None => "0".to_owned(),
        };
        ret += &format!("W{i}: {bal}\n");
    }

    Ok(ret)
}

pub fn get_balance_keyboard() -> InlineKeyboardMarkup {
    InlineKeyboardMarkup::new(vec![
        vec![get_main_menu_button(), get_faq_button()],
        // vec![InlineKeyboardButton::callback(
        //     "Add Token",
        //     &format!("{REPLY_ACT}_{SET_TOKEN_DB}"),
        // )],
    ])
}
