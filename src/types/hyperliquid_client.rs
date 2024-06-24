use crate::AddressForBot;
use crate::{globals::NETWORK, TOKEN_LIST};
use anyhow::{anyhow, Result};
use ethers::types::Address;
use futures::future::join_all;
use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
struct RequestWithUser {
    #[serde(rename = "type")]
    request_type: String,
    user: String,
}

#[derive(Deserialize, Debug)]
pub struct OpenOrder {
    pub coin: String,
    #[serde(rename = "limitPx")]
    pub limit_px: String,
    pub oid: u64,
    pub side: String,
    pub sz: String,
    pub timestamp: u64,
}

pub enum HyperLiquidNetwork {
    #[allow(dead_code)]
    Testnet,
    Mainnet,
}

impl HyperLiquidNetwork {
    pub fn get_client() -> HyperLiquidClient {
        let url = match *NETWORK.lock().unwrap() {
            HyperLiquidNetwork::Testnet => "https://testnet.api.hyperliquid.xyz/info".to_string(),
            HyperLiquidNetwork::Mainnet => "https://api.hyperliquid.xyz/info".to_string(),
        };
        HyperLiquidClient { url }
    }
}
#[derive(Serialize, Clone)]
pub struct HyperLiquidClient {
    url: String,
}

/// Spot
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct SpotMetaRequest {
    #[serde(rename = "type")]
    request_type: String,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TokenInfo {
    pub name: String,
    pub sz_decimals: u32,
    pub wei_decimals: u32,
    pub index: u32,
    pub token_id: String,
    pub is_canonical: bool,
}

impl TokenInfo {
    pub fn usdc_pair_name(&self) -> Option<String> {
        match self.index {
            x if x == 0 => None,
            y if y == 1 => Some("PURR/USDC".to_owned()),
            z => {
                let pair_name = "@".to_owned() + &(z - 1).to_string();
                Some(pair_name)
            }
        }
    }
}

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
struct UniverseInfo {
    pub name: String,
    pub tokens: Vec<u32>,
    pub index: u32,
    pub is_canonical: Option<bool>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SpotMetaResponse {
    pub tokens: Vec<TokenInfo>,
    #[allow(dead_code)]
    universe: Vec<UniverseInfo>,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
struct DayData {
    prev_day_px: String,
    day_ntl_vlm: String,
    mark_px: String,
    mid_px: String,
    circulating_supply: String,
    coin: String,
}

#[derive(Deserialize, Serialize)]
pub struct SimpleRequest {
    #[serde(rename = "type")]
    request_type: String,
}

#[derive(Deserialize, Debug, Default)]
pub struct Balance {
    pub coin: String,
    pub hold: String,
    pub total: String,
}

#[derive(Deserialize, Debug)]
pub struct BalancesResponse {
    pub balances: Vec<Balance>,
}

#[derive(Deserialize, Debug)]
pub struct TokenPrice {
    pub name: String,
    pub price: String,
}

#[derive(Deserialize, Debug)]
pub struct OpenOrdersResponse {
    pub orders: Vec<OpenOrder>,
}

// #[derive(Debug, Serialize, Deserialize)]
// struct AssetCtxResponse {
//     universe: Vec<Universe>,
//     tokens: Vec<Token>,
//     coinData: Vec<CoinData>,
// }

impl HyperLiquidClient {
    pub async fn fetch_spot_meta(&self) -> Result<SpotMetaResponse> {
        let client = Client::new();
        let request_body = SpotMetaRequest {
            request_type: "spotMeta".to_string(),
        };

        let response = client.post(&self.url).json(&request_body).send().await?;

        if response.status().is_success() {
            let spot_meta = response.json::<SpotMetaResponse>().await?;
            Ok(spot_meta)
        } else {
            Err(anyhow::anyhow!(
                "Failed to fetch spotMeta: {}",
                response.status()
            ))
        }
    }

    pub async fn fetch_spot_balance_for_addresses(
        self,
        addresses: &Vec<Address>,
    ) -> Result<Vec<Vec<Balance>>> {
        let client = Client::new();
        let futures = addresses
            .iter()
            .map(|&address| self.fetch_spot_balances(&client, address));
        let mut orders_responses = Vec::new();
        for result in join_all(futures).await {
            orders_responses.push(result?);
        }
        Ok(orders_responses)
    }
    pub async fn fetch_price_for_token(self, token: &str) -> Result<String> {
        let token = TOKEN_LIST.get_result(token)?;

        let prices = self.fetch_prices().await?;

        Ok(prices
            .iter()
            .find(|&x| x.name == token.name)
            .ok_or(anyhow!("Couldn't fin for {}", token.name))?
            .price
            .clone())
    }
    pub async fn fetch_prices(&self) -> Result<Vec<TokenPrice>> {
        let client = Client::new();
        let request_body = SimpleRequest {
            request_type: "spotMetaAndAssetCtxs".to_string(),
        };

        let response = client
            .post(&self.url)
            .header("Content-Type", "application/json")
            .json(&request_body)
            .send()
            .await?;

        if response.status().is_success() {
            let (spot_meta, day_data) = response.json::<(SpotMetaResponse, Vec<DayData>)>().await?;
            let mut ret: Vec<TokenPrice> = Vec::new();
            for (i, token) in spot_meta.tokens.iter().enumerate().skip(1) {
                let data = match day_data.get(i - 1).cloned() {
                    Some(data) => data,
                    None => DayData {
                        day_ntl_vlm: "Nan".to_owned(),
                        mark_px: "Nan".to_owned(),
                        mid_px: "Nan".to_owned(),
                        prev_day_px: "Nan".to_owned(),
                        circulating_supply: "Nan".to_owned(),
                        coin: "Nan".to_string(),
                    },
                };
                ret.push(TokenPrice {
                    name: token.name.clone(),
                    price: data.mark_px.clone(),
                });
            }
            Ok(ret)
        } else {
            Err(anyhow!("Failed to get prices"))
        }
    }
    pub async fn fetch_spot_balances(
        &self,
        client: &Client,
        address: Address,
    ) -> Result<Vec<Balance>> {
        let request_body = RequestWithUser {
            request_type: "spotClearinghouseState".to_string(),
            user: address.to_full_string(),
        };

        let response = client
            .post(&self.url)
            .header("Content-Type", "application/json")
            .json(&request_body)
            .send()
            .await?;

        if response.status().is_success() {
            let balances = response.json::<BalancesResponse>().await?;

            Ok(balances.balances)
        } else {
            Err(anyhow!(
                "Failed to get balances for {}: {}",
                address,
                response.status()
            ))
        }
    }
    pub async fn fetch_open_orders_for_addresses(
        self,
        addresses: &Vec<Address>,
    ) -> Result<Vec<Vec<OpenOrder>>> {
        let client = Client::new();
        let futures = addresses
            .iter()
            .map(|&address| self.fetch_open_orders(&client, address));
        let results = join_all(futures).await;
        let mut orders_responses = Vec::new();
        for result in results {
            orders_responses.push(result?);
        }
        Ok(orders_responses)
    }
    async fn fetch_open_orders(&self, client: &Client, user: Address) -> Result<Vec<OpenOrder>> {
        let request_body = RequestWithUser {
            request_type: "openOrders".to_string(),
            user: user.to_full_string(),
        };

        let response = client
            .post(&self.url)
            .header("Content-Type", "application/json")
            .json(&request_body)
            .send()
            .await?;

        if response.status().is_success() {
            let orders = response.json::<Vec<OpenOrder>>().await?;
            Ok(orders)
        } else {
            Err(anyhow!(
                "Failed to get open orders for {}: {}",
                user,
                response.status()
            ))
        }
    }
}
