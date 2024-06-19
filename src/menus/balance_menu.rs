use crate::handlers::constants_callbacks::BALANCES_MENU;
use crate::types::hyperliquid_client::HyperLiquidNetwork;
use crate::{globals::*, vec_3_p_keys_to_address};
use anyhow::Result;
use ethers::types::Address;
use std::collections::HashMap;
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
    let client_2 = HyperLiquidNetwork::get_client();
    let mut ret = String::new();
    let (balances, prices) = tokio::join!(
        client.fetch_spot_balance_for_addresses(&addresses),
        client_2.fetch_prices()
    );
    let mut hash_map: HashMap<String, f64> = HashMap::new();

    // let (prices, balances) = (prices?, balances?);
    let prices = prices?;

    for (i, wallet) in balances?.iter().enumerate() {
        ret += &format!("\n<b>Wallet {}-------\n</b>", i + 1);
        let mut entered_loop = false;
        for balance in wallet.iter() {
            let (bal_locked, bal_total): (f64, f64) = (
                balance.hold.clone().parse()?,
                balance.total.clone().parse()?,
            );
            let bal_free = bal_total - bal_locked;

            let mut price = 0.0;

            for price_info in prices.iter() {
                if price_info.name == balance.coin {
                    price = price_info.price.parse::<f64>()?;
                }
            }
            let total_price = match balance.coin.as_ref() {
                "USDC" => bal_total,
                _ => bal_total * price,
            };
            entered_loop = true;
            ret += &format!(
                "{} :  {:.2} (🔒{:.2}): <b>{:.2}$</b>\n",
                balance.coin.to_ascii_uppercase(),
                bal_free,
                bal_locked,
                total_price
            );
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
use crate::get_back_and_faq_banner;
use crate::get_refresh_button;
use crate::handlers::constants_callbacks::TRADE_MENU;
pub fn get_balance_keyboard() -> InlineKeyboardMarkup {
    InlineKeyboardMarkup::new(vec![
        get_back_and_faq_banner(TRADE_MENU),
        vec![get_refresh_button(BALANCES_MENU)],
    ])
}
