use crate::types::hyperliquid_client::HyperLiquidNetwork;
use crate::{get_faq_button, get_main_menu_button};
use crate::{globals::*, vec_3_p_keys_to_address};
use anyhow::Result;
use ethers::types::Address;

use teloxide::{types::InlineKeyboardMarkup, types::User};

pub async fn orders_menu(user: &User) -> anyhow::Result<(String, InlineKeyboardMarkup)> {
    let pks = WALLETS_PKEY.get_result(user.id)?;
    let addresses = vec_3_p_keys_to_address(&pks);
    let mut text = format!(
        "<b>🤖 Hyperliquid 
    Your Orders </b> \n"
    );
    debug!("Display  full order");
    text += &display_full_order(addresses).await?;

    Ok((text, get_orders_keyboard()))
}

pub async fn display_balance(addresses: Vec<Address>) -> Result<String> {
    Ok("Display_balance".to_owned())
}
pub async fn display_full_order(addresses: Vec<Address>) -> Result<String> {
    let client = HyperLiquidNetwork::get_client();
    let mut ret = String::new();
    let balances = client.fetch_open_orders_for_addresses(&addresses).await?;
    for (i, wallet) in balances.iter().enumerate() {
        ret += &format!("\n<b>Wallet {i}-------\n</b>");
        let mut entered_loop = false;
        for orders in wallet.iter() {
            entered_loop = true;
            let type_order = match orders.side.as_ref() {
                "B" => "Buy",
                "S" => "Sell",
                _ => "Bizarre",
            };
            ret += &format!(
                "{} {}{} at {} \n",
                type_order, orders.sz, orders.coin, orders.limit_px,
            )
        }
        if !entered_loop {
            ret += &format!("<i>Empty</i>");
        }
    }
    Ok(ret)
}

pub fn get_orders_keyboard() -> InlineKeyboardMarkup {
    InlineKeyboardMarkup::new(vec![
        vec![get_main_menu_button(), get_faq_button()],
        // vec![InlineKeyboardButton::callback(
        //     "Add Token",
        //     &format!("{REPLY_ACT}_{SET_TOKEN_DB}"),
        // )],
    ])
}
