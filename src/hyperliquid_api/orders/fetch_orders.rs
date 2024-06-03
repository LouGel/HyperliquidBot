use crate::{types::*, PKeyHandler, WALLETS_PKEY};
// use crate::check_and_id
use crate::utils::format_float;
use anyhow::{anyhow, Result};
use ethers::types::*;
use ethers::utils::format_units;
use futures::future::join_all;
use reqwest::Client;
use serde::Deserialize;
use teloxide::types::UserId;
use tokio::time::{sleep, Duration};

#[derive(Debug, Deserialize)]
struct ApiOrderResponse {
    // pub code: i32,
    // pub message: String,
    pub data: Data,
}

#[derive(Debug, Deserialize)]
struct Data {
    pub orders: Vec<Order>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Order {
    pub id: u64,
    // pub chain_id: String,
    // pub nonce: u64,
    pub maker_asset: Address,
    pub taker_asset: Address,
    pub contract_address: Address,
    // pub order_hash: H256,
    pub maker_asset_symbol: String,
    pub taker_asset_symbol: String,
    // pub maker_asset_logo_url: String,
    // pub taker_asset_logo_url: String,
    pub maker_asset_decimals: u8,
    pub taker_asset_decimals: u8,
    #[serde(deserialize_with = "deserialize_u256_from_decimal_string")]
    pub making_amount: U256,
    #[serde(deserialize_with = "deserialize_u256_from_decimal_string")]
    pub taking_amount: U256,
    #[serde(deserialize_with = "deserialize_u256_from_decimal_string")]
    pub filled_making_amount: U256,
    #[serde(deserialize_with = "deserialize_u256_from_decimal_string")]
    pub filled_taking_amount: U256,
    #[serde(deserialize_with = "order_status_from_str")]
    pub status: OrderStatus,
    pub created_at: u64,
    pub expired_at: u64,
    pub operator_signature_expired_at: u64,
}

async fn fetch_orders_by_address(
    chain_id: u64,
    maker: Address,
    status: &OrderStatus,
) -> Result<Vec<Order>> {
    dotenv::dotenv().ok();
    debug!("In fetch_order");
    let attempt = 4;
    let client = Client::new();
    let url = &format!(
        "https://limit-order.kyberswap.com/read-ks/api/v1/orders?chainId={}&maker={:#x}&status={}",
        chain_id,
        maker,
        status.as_str()
    );
    debug!("Url : {}", url);
    for _ in 0..attempt {
        let response = client
            .get(url)
            .header("User-Agent", "curl/7.64.1")
            .header("x-client-id", "OmniBotX")
            .send()
            .await;

        match response {
            Ok(resp) => match resp.status().is_success() {
                true => {
                    let body = resp.text().await.expect("Failed to read response text");
                    let api_response: ApiOrderResponse =
                        serde_json::from_str(&body).map_err(|e| {
                            anyhow!("Failed to parse response: {} with body {}", e, body)
                        })?;
                    // debug!("Orders {:?}", api_response.data.orders);
                    return Ok(api_response.data.orders);
                }
                false => {
                    let status = resp.status();
                    let body = resp.text().await.unwrap_or_default();
                    return Err(anyhow!(
                        "Wrong status  for fetch orders: {}, status :{}",
                        body,
                        status.as_u16()
                    ));
                }
            },
            Err(e) => {
                error!("Request failed: {:?}", e);
                sleep(Duration::from_millis(100)).await;
            }
        }
    }

    Err(anyhow!("Failed after {attempt} attempts"))
}
pub async fn fetch_orders_by_user_id(
    chain_id: u64,
    user_id: UserId,
    status: OrderStatus,
) -> Result<Vec<Vec<Order>>> {
    let pks = WALLETS_PKEY.get_result(user_id)?;
    let orders_query: Vec<_> = pks
        .into_iter()
        .map(|x| fetch_orders_by_address(chain_id, x.to_address(), &status))
        .collect();
    let mut res = Vec::new();
    let raw_results = join_all(orders_query).await;
    for result in raw_results {
        res.push(result?)
    }
    Ok(res)
}

pub fn format_orders_for_text_msg(
    orders_for_wallet: Vec<Vec<Order>>,
    order_type_: Option<OrderType>,
) -> Result<String> {
    todo!("format_orders_for_text_msg")
}

pub fn format_order_for_line(wallet_no: usize, order: &Order) -> anyhow::Result<String> {
    let maker_amount = format_units(order.making_amount, order.maker_asset_decimals as u32)?;
    let taker_amount = format_units(order.taking_amount, order.taker_asset_decimals as u32)?;
    Ok(format!(
        "W{}->id=({}) [{}{} for {}{}]\n <i>{}</i>",
        wallet_no,
        order.id,
        format_float(maker_amount, 4),
        order.maker_asset_symbol,
        format_float(taker_amount, 4),
        order.taker_asset_symbol,
        order.status.as_str(),
    ))
}

#[derive(Debug, Deserialize)]
pub enum OrderStatus {
    Active,
    Open,
    PartiallyFilled,
    Closed,
    Filled,
    Cancelled,
    Expired,
}

use std::str::FromStr;

use super::OrderType; // Ensure you have anyhow in your Cargo.toml

impl FromStr for OrderStatus {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        match s {
            "active" => Ok(OrderStatus::Active),
            "open" => Ok(OrderStatus::Open),
            "closed" => Ok(OrderStatus::Closed),
            "filled" => Ok(OrderStatus::Filled),
            "partially_filled" => Ok(OrderStatus::PartiallyFilled),
            "cancelled" => Ok(OrderStatus::Cancelled),
            "expired" => Ok(OrderStatus::Expired),
            _ => Err(anyhow!("Invalid order status")),
        }
    }
}
impl OrderStatus {
    fn as_str(&self) -> &'static str {
        match *self {
            OrderStatus::Active => "active",
            OrderStatus::Open => "open",
            OrderStatus::PartiallyFilled => "partially_filled",
            OrderStatus::Closed => "closed",
            OrderStatus::Filled => "filled",
            OrderStatus::Cancelled => "cancelled",
            OrderStatus::Expired => "expired",
        }
    }
}
fn order_status_from_str<'de, D>(deserializer: D) -> Result<OrderStatus, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    s.parse::<OrderStatus>().map_err(serde::de::Error::custom)
}

pub fn deserialize_u256_from_decimal_string<'de, D>(deserializer: D) -> Result<U256, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    U256::from_dec_str(&s).map_err(serde::de::Error::custom)
}
