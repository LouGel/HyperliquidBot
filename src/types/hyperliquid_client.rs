use reqwest::Client;
use serde::{Deserialize, Serialize};
use teloxide::types::OrderInfo;
// use std::error::Error;
use crate::globals::NETWORK;
// use crate::hyperliquid_api::fetch_orders::OpenOrdersResponse;
use crate::AddressForBot;
use anyhow::{anyhow, Error, Result};
use ethers::types::Address;
use futures::future::join_all;
use serde::*;

#[derive(Serialize)]
struct OpenOrdersRequest {
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
#[derive(Serialize)]
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

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct UniverseInfo {
    name: String,
    tokens: Vec<u32>,
    index: u32,
    is_canonical: bool,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SpotMetaResponse {
    pub tokens: Vec<TokenInfo>,
    universe: Vec<UniverseInfo>,
}
#[derive(Serialize)]
pub struct BalancesRequest {
    #[serde(rename = "type")]
    request_type: String,
    user: String,
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
pub struct OpenOrdersResponse {
    pub orders: Vec<OpenOrder>,
}

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
    pub async fn fetch_spot_balances(
        &self,
        client: &Client,
        address: Address,
    ) -> Result<Vec<Balance>> {
        let request_body = BalancesRequest {
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
        let request_body = OpenOrdersRequest {
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
            // debug!("{:#?}", response.json().await?);
            // todo!()
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
