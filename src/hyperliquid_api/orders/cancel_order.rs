use regex::Regex;

use crate::globals::*;
use hyperliquid_rust_sdk::{BaseUrl, ClientCancelRequest, ExchangeClient};

use crate::traits::PKeyHandler;
use anyhow::Result;

use teloxide::types::*;

pub async fn cancel_from_menu(user: &User, text: &str, order_number: String) -> Result<()> {
    let order_info = extract_order_info(text, order_number)?;
    let pk = WALLETS_PKEY.get_pk_for_index(user.id, order_info.wallet_index)?;
    let str = pk.to_hex_string();
    let asset = TOKEN_LIST
        .get_result(&order_info.token_name)?
        .usdc_pair_name()
        .ok_or(anyhow::anyhow!(
            "Error Asset{} pair found",
            order_info.token_name
        ))?;
    let exchange_client = ExchangeClient::new(
        None,
        str.parse().unwrap(),
        Some(BaseUrl::Mainnet),
        None,
        None,
        None,
    )
    .await?;

    let cancel = ClientCancelRequest {
        asset,
        oid: order_info.order_id,
    };
    exchange_client.cancel(cancel, None).await?;
    Ok(())
}

#[derive(Debug)]
struct CancelOrderInfo {
    wallet_index: usize,
    order_type: String,
    token_name: String,
    order_id: u64,
}

fn extract_order_info(text: &str, order_number: String) -> Result<CancelOrderInfo> {
    let wallet_re = Regex::new(r"Wallet (\d+)-------")?;
    let order_re = Regex::new(r"(\d+)\.(Buy|Sell) \d+\.\d+\[(\w+)\] .* \((\d+)\)")?;
    let mut current_wallet = None;

    for line in text.lines() {
        if let Some(wallet_caps) = wallet_re.captures(line) {
            current_wallet = Some(wallet_caps[1].parse::<usize>()? - 1);
        } else if let Some(order_caps) = order_re.captures(line) {
            if order_caps[1] == order_number {
                if let Some(wallet_index) = current_wallet {
                    return Ok(CancelOrderInfo {
                        wallet_index,
                        order_type: order_caps[2].to_string(),
                        token_name: order_caps[3].to_string(),
                        order_id: order_caps[4].parse::<u64>()?,
                    });
                }
            }
        }
    }

    Err(anyhow::anyhow!("Order number {} not found.", order_number))
}
