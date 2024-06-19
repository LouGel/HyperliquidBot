use super::get_data_from_hyperliquid_response;
use crate::handlers::constants_callbacks::{AMOUNT_PLAIN, PRICE_WANTED, TOKEN_NAME, WALLET};
use crate::traits::{OmnixString, PKeyHandler};
use crate::HyperLiquidNetwork;
use crate::{globals::*, InlineKeyBoardHandler};
use anyhow::{anyhow, Result};
use hyperliquid_rust_sdk::{BaseUrl, ClientLimit, ClientOrder, ClientOrderRequest, ExchangeClient};

use teloxide::prelude::*;
use teloxide::types::*;

pub async fn order_from_menu(
    _bot: &Bot,
    user: &User,
    menu: InlineKeyboardMarkup,
) -> Result<String> {
    let (wallet_index, order) = get_wallet_index_and_order_from_markup(&menu)
        .await
        .map_err(|e| anyhow::anyhow!("Datas in order are not set correctly {}", e.to_string()))?;
    let pk = WALLETS_PKEY.get_pk_for_index(user.id, wallet_index)?;
    let str = pk.to_hex_string();
    let exchange_client = ExchangeClient::new(
        None,
        str.parse().unwrap(),
        Some(BaseUrl::Mainnet),
        None,
        None,
        None,
    )
    .await
    .unwrap();

    let response = exchange_client.order(order, None).await?;
    get_data_from_hyperliquid_response(response)
}

async fn get_wallet_index_and_order_from_markup(
    keyboard: &InlineKeyboardMarkup,
) -> Result<(usize, ClientOrderRequest)> {
    let pren_name = keyboard.get_result_from_callback_fct(TOKEN_NAME)?;

    let name = &pren_name
        .split(" ")
        .nth(0)
        .ok_or(anyhow::anyhow!("Couldn't get token name {pren_name}"))?;
    let wallet_index = keyboard
        .get_result_from_checked_callback_fct(WALLET)?
        .clean_and_parse_to_usize()?
        .checked_sub(1)
        .ok_or(anyhow!(" Wrong wallet index"))?;

    let (is_buy, is_limit) = keyboard.get_which_order_type()?;
    let token = TOKEN_LIST.get_result(name)?;
    let asset = token
        .usdc_pair_name()
        .ok_or(anyhow::anyhow!("Wrong token {}", token.name))?;
    let client = HyperLiquidNetwork::get_client();
    let limit_px: f64 = match is_limit {
        true => keyboard
            .get_value_from_callback_fct(PRICE_WANTED)
            .ok_or(anyhow!("Could get price from limit menu"))?
            .clean_and_parse_to_float()?,
        false => {
            let mut price = client.fetch_price_for_token(&name).await?.parse::<f64>()?;
            if is_buy {
                price *= 1.02
            } else {
                price *= 0.98
            };
            format_sz(price, token.sz_decimals).parse()?
        }
    };

    let pre_sz: f64 = keyboard
        .get_value_from_callback_fct(AMOUNT_PLAIN)
        .ok_or(anyhow!("Could get amount from menu"))?
        .clean_and_parse_to_float()?;
    let sz = match is_buy {
        true => format_sz(pre_sz / limit_px, token.sz_decimals).parse::<f64>()?,
        false => pre_sz,
    };
    let order_type = match is_limit {
        true | false => ClientOrder::Limit(ClientLimit {
            tif: "Gtc".to_string(),
        }),
    };
    let order = ClientOrderRequest {
        asset,
        is_buy,
        reduce_only: false,
        limit_px,
        sz,
        cloid: None,
        order_type,
    };
    info!("Order {:#?}", order);
    Ok((wallet_index, order))
}

pub fn format_sz(number: f64, decimals: u32) -> String {
    let d = decimals as usize;
    let format_string = format!("{}", number);
    let two_args: Vec<&str> = format_string.split('.').collect();
    let new_dec = match two_args[1].len() > d {
        true => two_args[1][..d].to_owned(),
        false => two_args[1].to_owned(),
    };
    let two_new_args = vec![two_args[0].to_owned(), new_dec];
    two_new_args.join(".")
}
