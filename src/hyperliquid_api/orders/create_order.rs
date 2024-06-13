use ethers::signers::LocalWallet;
use log::info;
use serde::{Deserialize, Serialize};

use anyhow::Result;
use ethers_core::k256::ecdsa::SigningKey;
use ethers_core::k256::{elliptic_curve::SecretKey, Secp256k1};
use hyperliquid_rust_sdk::{
    BaseUrl, ClientCancelRequest, ClientLimit, ClientOrder, ClientOrderRequest, ExchangeClient,
    ExchangeDataStatus, ExchangeResponseStatus,
};
use std::{thread::sleep, time::Duration};

use crate::PKeyHandler;

async fn create_order(pk: SecretKey<Secp256k1>, is_limit: bool, is_buy: bool) -> Result<u64> {
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

    let order = ClientOrderRequest {
        asset: "@6".to_string(),
        is_buy: true,
        reduce_only: false,
        limit_px: 0.001,
        sz: 10000.0,
        cloid: None,
        order_type: ClientOrder::Limit(ClientLimit {
            tif: "Gtc".to_string(),
        }),
    };
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

    Ok(oid)
}
