use std::str::FromStr;

use crate::globals::*;
use crate::menus::buy_menu::*;
use crate::traits::{PKeyHandler, TxLink};
use crate::AddressForBot;
use anyhow::Result;
use hyperliquid_rust_sdk::{
    BaseUrl, ClientCancelRequest, ClientLimit, ClientOrder, ClientOrderRequest, ExchangeClient,
    ExchangeDataStatus, ExchangeResponseStatus,
};
use teloxide::prelude::*;
use teloxide::types::*;
use url::Url;
pub async fn order_from_menu(_bot: &Bot, user: &User, menu: InlineKeyboardMarkup) -> Result<Url> {
    let (wallet_no, order) = get_values_from_buy_markup(menu)?;
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
    Ok(Url::from_str(&oid.to_string())?)
}
