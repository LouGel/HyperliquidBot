use std::str::FromStr;

use crate::handlers::constants_callbacks::{AMOUNT_PLAIN, TOKEN_NAME};
use crate::handlers::constants_callbacks::{PRICE_WANTED, WALLET};
use crate::traits::OmnixString;
use crate::{globals::*, InlineKeyBoardHandler};
use hyperliquid_rust_sdk::{
    BaseUrl, ClientCancelRequest, ClientLimit, ClientOrder, ClientOrderRequest, ClientTrigger,
    ExchangeClient, ExchangeDataStatus, ExchangeResponseStatus,
};

use crate::traits::PKeyHandler;
use anyhow::Result;
use ethers::abi::token;
use ethers_signer::wallet;

use teloxide::prelude::*;
use teloxide::types::*;
use url::Url;
pub async fn order_from_menu(_bot: &Bot, user: &User, menu: InlineKeyboardMarkup) -> Result<()> {
    let (wallet_no, order) = get_wallet_no_and_order_from_markup(&menu)?;
    let pk = WALLETS_PKEY.get_pk_for_index(user.id, wallet_no)?;
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
    let response = match response {
        ExchangeResponseStatus::Ok(exchange_response) => exchange_response,
        ExchangeResponseStatus::Err(e) => {
            return Err(anyhow::anyhow!("error with exchange response: {e}"))
        }
    };
    let status = response.data.unwrap().statuses[0].clone();
    let oid = match status {
        ExchangeDataStatus::Filled(order) => order.oid,
        ExchangeDataStatus::Resting(order) => order.oid,
        _ => panic!("Error: {status:?}"),
    };
    Ok(())
}

fn get_wallet_no_and_order_from_markup(
    keyboard: &InlineKeyboardMarkup,
) -> Result<(usize, ClientOrderRequest)> {
    let pren_name = keyboard.get_result_from_callback_fct(TOKEN_NAME)?;

    let name = &pren_name
        .split(" ")
        .nth(0)
        .ok_or(anyhow::anyhow!("Couldn't get token name {pren_name}"))?;
    let wallet = keyboard
        .get_result_from_checked_callback_fct(WALLET)?
        .clean_and_parse_to_usize()?
        - 1;

    let (is_buy, is_limit) = keyboard.get_which_order_type()?;
    let token = TOKEN_LIST.get_result(name)?;
    let limit_px: f64 = keyboard
        .get_value_from_callback_fct(PRICE_WANTED)
        .unwrap_or("0.0".to_owned())
        .parse()?;
    let sz: f64 = keyboard
        .get_value_from_callback_fct(AMOUNT_PLAIN)
        .unwrap_or("0.0".to_owned())
        .parse()?;
    let order_type = match is_limit {
        true => ClientOrder::Limit(ClientLimit {
            tif: "Gtc".to_string(),
        }),
        false => ClientOrder::Trigger(ClientTrigger {
            is_market: !is_limit,
            tpsl: "idk".to_owned(),
            trigger_px: 0.0,
        }),
    };
    debug!("sz = {}", sz);

    let asset = token
        .usdc_pair_name()
        .ok_or(anyhow::anyhow!("Wrong token {}", token.name))?;

    let order = ClientOrderRequest {
        asset,
        is_buy,
        reduce_only: false,
        limit_px,
        sz,
        cloid: None,
        order_type,
    };
    Ok((wallet, order))
}
