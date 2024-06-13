use crate::types::hyperliquid_client::*;
use crate::types::Action;
use crate::types::*;
use anyhow::{anyhow, Result};
use ethers::prelude::*;
use ethers_core::k256::elliptic_curve::SecretKey;
use ethers_core::k256::Secp256k1;
use lazy_static::lazy_static;
use once_cell::sync::OnceCell;
use sqlx::{Pool, Postgres};
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};
use teloxide::types::UserId;

// pub async fn get_token_list() -> Result {}

pub fn toggle_test() {
    let mut network = NETWORK.lock().unwrap();
    *network = match *network {
        HyperLiquidNetwork::Mainnet => HyperLiquidNetwork::Testnet,
        HyperLiquidNetwork::Testnet => HyperLiquidNetwork::Mainnet,
    };
}

lazy_static! {

    // Function to initialize the TOKEN_LIST
    pub static ref TOKEN_LIST: Arc<Mutex<HashMap<String, Arc<TokenInfo>>>> = Arc::new(Mutex::new(HashMap::new()));
    pub static ref POOL: OnceCell<Arc<Pool<Postgres>>> = OnceCell::new();

    // MUTABLE global vars
    pub static ref NETWORK: Mutex<HyperLiquidNetwork> = Mutex::new(HyperLiquidNetwork::Mainnet);
    pub static ref PASSWD: Arc<Mutex<HashMap<i64, String>>> = Arc::new(Mutex::new(HashMap::new()));
    pub static ref WALLETS_PKEY: Arc<Mutex<HashMap<i64, Vec<SecretKey<Secp256k1>>>>> =
        Arc::new(Mutex::new(HashMap::new()));
    pub static ref REPLY_ACTION: Arc<Mutex<HashMap<i64, Action>>> =
        Arc::new(Mutex::new(HashMap::new()));
}

impl TOKEN_LIST {
    pub fn get_result(&self, token_name: String) -> anyhow::Result<Arc<TokenInfo>> {
        Ok(self
            .lock()
            .map_err(|e| {
                anyhow!(
                    "Poison lock in TokenInfo get for {} --> {}",
                    token_name,
                    e.to_string()
                )
            })?
            .get(&token_name)
            .ok_or(anyhow!("Couldn't access pk for user {}", token_name))?
            .clone())
    }
}

impl WALLETS_PKEY {
    pub fn get_result(&self, user_id: UserId) -> anyhow::Result<Vec<SecretKey<Secp256k1>>> {
        let parsed_id = user_id.0 as i64;
        Ok(self
            .lock()
            .map_err(|e| {
                anyhow!(
                    "Poison lock in Wallet WALLETS_PKEY for {} --> {}",
                    parsed_id,
                    e.to_string()
                )
            })?
            .get(&parsed_id)
            .ok_or(anyhow!("Couldn't access pk for user {}", parsed_id))?
            .clone())
    }
    pub fn get_pk_for_index(
        &self,
        user_id: UserId,
        index: usize,
    ) -> anyhow::Result<SecretKey<Secp256k1>> {
        Ok(self
            .get_result(user_id)?
            .get(index)
            .ok_or(anyhow!(
                "Couldn't access pk {} for user {}",
                index,
                user_id.0
            ))?
            .to_owned())
    }
}

pub fn get_pool() -> Arc<Pool<Postgres>> {
    POOL.get().expect("Pool has not been initialized").clone()
}

pub const WHITEPAPER_URL: &str = "https://whitepaper.wagmi.io/info/";
#[macro_export]
macro_rules! address {
    ($expr:expr) => {{
        Address::from_str($expr).unwrap()
    }};
}

pub const DEAD_CALLBACK: &str = "!";
