use crate::get_pool;
use crate::traits::OmnixNumber;
use crate::types::Tokens;
use crate::PoolOperation;
use crate::{AddressForBot, TOKEN_LIST};
use anyhow::{anyhow, Result};
use ethers::providers::Middleware;
use ethers::types::Address;
use ethers::types::U256;
use ethers::utils::format_units;
use ethers_core::types::H160;
use reqwest::header::{HeaderMap, HeaderValue, ACCEPT};
use serde::{Deserialize, Serialize};

use std::collections::HashMap;

use tokio::join;

use futures::future::{join, join_all};

#[derive(Serialize)]
struct RpcRequest<'a> {
    id: u32,
    jsonrpc: &'a str,
    method: &'a str,
    params: Vec<String>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct TokenBalanceRaw {
    // #[serde(rename = "contractAddress")]
    token_address: Address,
    // #[serde(rename = "tokenBalance")]
    // #[serde(deserialize_with = "deserialize_u256_from_decimal_string")]
    balance: U256,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TokenBalance {
    pub address: String,
    pub name: String,
    pub balance: String,
    pub is_from_omnix: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RpcFullBalance {
    cursor: Option<()>,
    page: u32,
    page_size: u32,
    result: Vec<TokenFullInfo>,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct TokenFullInfo {
    pub token_address: Address,
    pub symbol: String,
    pub name: String,
    pub logo: Option<String>,
    pub thumbnail: Option<String>,
    pub decimals: u32,
    pub balance: String, // Using String to handle large numbers
    pub possible_spam: bool,
    pub verified_contract: bool,
    pub balance_formatted: String,
    pub usd_price: Option<f64>,
    pub usd_price_24hr_percent_change: Option<f64>,
    pub usd_price_24hr_usd_change: Option<f64>,
    pub usd_value: Option<f64>,
    pub usd_value_24hr_usd_change: Option<f64>,
    pub native_token: bool,
    pub portfolio_percentage: f64,
}
