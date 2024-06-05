use reqwest::Client;
use serde::{Deserialize, Serialize};
// use std::error::Error;
use crate::globals::NETWORK;
use crate::hyperliquid_api::fetch_orders::OpenOrdersResponse;
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
struct OpenOrder {
    coin: String,
    #[serde(rename = "limitPx")]
    limit_px: String,
    oid: u64,
    side: String,
    sz: String,
    timestamp: u64,
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
struct HyperLiquidClient {
    url: String,
}

/// Spot
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct SpotMetaRequest {
    #[serde(rename = "type")]
    request_type: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TokenInfo {
    name: String,
    sz_decimals: u32,
    wei_decimals: u32,
    index: u32,
    token_id: Address,
    is_canonical: bool,
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
struct SpotMetaResponse {
    tokens: Vec<TokenInfo>,
    universe: Vec<UniverseInfo>,
}

impl HyperLiquidClient {
    pub async fn fetch_open_orders_for_addresses(
        self,
        addresses: Vec<Address>,
    ) -> Result<Vec<OpenOrdersResponse>> {
        let client = Client::new();
        let futures = addresses
            .iter()
            .map(|&address| self.fetch_open_orders(&client, address));
        let results = join_all(futures).await;
        let mut orders_responses = Vec::new();
        for result in results {
            orders_responses.push(result?);
        }
        todo!()
    }
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
    async fn fetch_open_orders(
        &self,
        client: &Client,
        user: Address,
    ) -> Result<OpenOrdersResponse> {
        let request_body = OpenOrdersRequest {
            request_type: "openOrders".to_string(),
            user: user.to_full_string(),
        };

        let response = client.post(&self.url).json(&request_body).send().await?;

        if response.status().is_success() {
            let orders = response.json::<OpenOrdersResponse>().await?;
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
