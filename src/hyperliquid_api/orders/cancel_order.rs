use regex::Regex;

use crate::globals::*;
use hyperliquid_rust_sdk::{BaseUrl, ClientCancelRequest, ExchangeClient, ExchangeResponseStatus};

use crate::traits::PKeyHandler;
use anyhow::{anyhow, Result};

use teloxide::types::*;

pub async fn cancel_from_menu(user: &User, text: &str, order_number: String) -> Result<String> {
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
    let resp = exchange_client.cancel(cancel, None).await?;
    get_data_from_hyperliquid_response(resp)
}
use hyperliquid_rust_sdk::ExchangeDataStatus;
pub fn get_data_from_hyperliquid_response(resp: ExchangeResponseStatus) -> Result<String> {
    let data = match resp {
        ExchangeResponseStatus::Ok(s) => s.data,
        ExchangeResponseStatus::Err(e) => return Err(anyhow!("{:?}", e)),
    };
    if let Some(d) = data {
        match d.statuses.get(0) {
            Some(d) => match d {
                ExchangeDataStatus::Success => Ok(format!("Order Succeed")),
                ExchangeDataStatus::WaitingForFill => Ok(format!("Order created")),
                ExchangeDataStatus::Filled(_) => Ok(format!("Order filled")),
                ExchangeDataStatus::Error(e) => Err(anyhow!("{}", e)),
                _ => Err(anyhow!("Unexpected error, fata status : {:?}", d)),
            },
            None => Err(anyhow!("no status in data".to_owned())),
        }
    } else {
        Err(anyhow!("no  data".to_owned()))
    }
}
#[derive(Debug)]
struct CancelOrderInfo {
    wallet_index: usize,
    token_name: String,
    order_id: u64,
}

// #[allow(dead_code)]
// order_type: String,

fn extract_order_info(text: &str, order_number: String) -> Result<CancelOrderInfo> {
    let wallet_re = Regex::new(r"Wallet (\d+):")?;
    let order_re = Regex::new(r"No (\d+): (\d+) \$([A-Z]+) at [\d.]+\$ oid\((\d+)\)")?;
    let mut current_wallet = None;

    for line in text.lines() {
        if let Some(wallet_caps) = wallet_re.captures(line) {
            current_wallet = Some(wallet_caps[1].parse::<usize>()? - 1);
        } else if let Some(order_caps) = order_re.captures(line) {
            if order_caps[1] == order_number {
                if let Some(wallet_index) = current_wallet {
                    let ret = CancelOrderInfo {
                        wallet_index,
                        // order_type: order_caps[2].to_string(),
                        token_name: order_caps[3].to_string(),
                        order_id: order_caps[4].parse::<u64>()?,
                    };
                    return Ok(ret);
                }
            }
        }
    }

    Err(anyhow::anyhow!("Order number {} not found.", order_number))
}
