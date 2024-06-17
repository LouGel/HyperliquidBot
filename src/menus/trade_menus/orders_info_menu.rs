use crate::handlers::constants_callbacks::*;
use crate::types::hyperliquid_client::HyperLiquidNetwork;
use crate::{get_faq_button, get_main_menu_button};
use crate::{globals::*, vec_3_p_keys_to_address};
use anyhow::Result;
use ethers::types::Address;
use teloxide::types::InlineKeyboardButton;

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

pub async fn display_full_order(addresses: Vec<Address>) -> Result<String> {
    let client = HyperLiquidNetwork::get_client();
    let mut ret = String::new();
    let balances = client.fetch_open_orders_for_addresses(&addresses).await?;
    for (i, wallet) in balances.iter().enumerate() {
        let mut num = 1;
        ret += &format!("\n<b>Wallet {}:\n</b>", i + 1);
        let mut entered_loop = false;
        let mut buy_orders = String::from("BUY: \n");
        let mut sell_orders = String::from("SELL: \n");
        for orders in wallet.iter() {
            entered_loop = true;

            let order_name = TOKEN_LIST.get_result(&orders.coin)?.name.clone();
            let order_str = &format!(
                "No {}: {} ${} at {}$ <i>oid({})</i>\n",
                num,
                orders.sz.trim_end_matches(".0").to_string(),
                order_name,
                orders.limit_px.trim_end_matches(".0").to_string(),
                orders.oid,
            );
            match orders.side.as_ref() {
                "B" => buy_orders += &order_str,
                "A" => sell_orders += &order_str,
                x => warn!("{x} type of order fetched"),
            };
            num += 1;
        }
        if !entered_loop {
            ret += &format!("<i>Empty</i>");
        } else {
            if buy_orders.contains("No") {
                ret += &buy_orders
            }
            if sell_orders.contains("No") {
                ret += &sell_orders
            }
        }
    }
    Ok(ret)
}

pub fn get_orders_keyboard() -> InlineKeyboardMarkup {
    InlineKeyboardMarkup::new(vec![
        vec![get_main_menu_button(), get_faq_button()],
        vec![InlineKeyboardButton::callback(
            "Cancel order",
            &format!("{REPLY_ACT}_{CANCEL_ORDER}"),
        )],
    ])
}
