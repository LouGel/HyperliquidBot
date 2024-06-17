use std::fmt::format;

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
        "<b>🤖 Hyperliquid </b>
<u>Your balances :</u> \n"
    );

    text += &display_full_balance(addresses, true).await?;

    Ok((text, get_balance_keyboard()))
}

pub async fn display_full_balance(addresses: Vec<Address>, need_total: bool) -> Result<String> {
    let client = HyperLiquidNetwork::get_client();
    let mut ret = String::new();
    let balances = client.fetch_spot_balance_for_addresses(&addresses).await?;
    for (i, wallet) in balances.iter().enumerate() {
        ret += &format!("\n<b>Wallet {}-------\n</b>", i + 1);
        let mut entered_loop = false;
        for balance in wallet.iter() {
            let (bal_locked, bal_total): (f64, f64) = (
                balance.hold.clone().parse()?,
                balance.total.clone().parse()?,
            );
            let bal_free = bal_total - bal_locked;
            let total_str = match need_total {
                false => "".to_owned(),
                true => format!("total:{:.2}", bal_total),
            };
            entered_loop = true;
            ret += &format!(
                "{} :  {:.2} (🔒{:.2}) {}\n",
                balance.coin, bal_free, bal_locked, total_str
            );
            if !entered_loop {
                ret += &format!("<i>Empty</i>");
            }
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
        let (bal_locked, bal_total): (f64, f64) = match usdc_balance.first() {
            Some(balance) => (
                balance.hold.clone().parse()?,
                balance.total.clone().parse()?,
            ),
            None => ("0.0".to_owned().parse()?, "0.0".to_owned().parse()?),
        };

        // println!("ba")
        let bal_free = bal_total - bal_locked;

        ret += &format!("W{}: {:.2} (🔒{:.2})\n", i + 1, bal_free, bal_locked);
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
