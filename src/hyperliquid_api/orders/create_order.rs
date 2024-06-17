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
    let (wallet_index, order) = get_wallet_index_and_order_from_markup(&menu).await?;
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
            .unwrap_or("0.0".to_owned())
            .parse()?,
        false => client.fetch_price_for_token(&name).await?.parse::<f64>()?,
    };

    let sz: f64 = keyboard
        .get_value_from_callback_fct(AMOUNT_PLAIN)
        .unwrap_or("0.0".to_owned())
        .parse()?;
    let order_type = match is_limit {
        true | false => ClientOrder::Limit(ClientLimit {
            tif: "Gtc".to_string(),
        }),
        // !!! To keep in case Hyperliquid wants to add Trigger trading for spots !!
        // false => {
        //     let (tpsl, trigger_px) = match is_buy {
        //         true => ("tp".to_owned(), limit_px * 0.95),
        //         false => ("sl".to_owned(), limit_px * 1.05),
        //     };

        //     ClientOrder::Trigger(ClientTrigger {
        //         is_market: true,
        //         tpsl,
        //         trigger_px,
        //     })
        // }
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
    Ok((wallet_index, order))
}
