use crate::init::update_token_list;
use crate::types::hyperliquid_client::*;
use crate::types::Action;
use anyhow::anyhow;
use chrono::Utc;
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

lazy_static! {

    // Function to initialize the TOKEN_LIST
    pub static ref TOKEN_LIST: Arc<Mutex<HashMap<String, Arc<TokenInfo>>>> = Arc::new(Mutex::new(HashMap::new()));
    pub static ref POOL: OnceCell<Arc<Pool<Postgres>>> = OnceCell::new();

    // MUTABLE global vars
    pub static ref NETWORK: Mutex<HyperLiquidNetwork> = Mutex::new(HyperLiquidNetwork::Mainnet);
    pub static ref PASSWD: Arc<Mutex<HashMap<i64, String>>> = Arc::new(Mutex::new(HashMap::new()));
    pub static ref WALLETS_PKEY: Arc<Mutex<HashMap<i64, Vec<SecretKey<Secp256k1>>>>> =
        Arc::new(Mutex::new(HashMap::new()));
    pub static ref REFRESH_RATE_PER_USER: Arc<Mutex<HashMap<UserId, u64>>> =
        Arc::new(Mutex::new(HashMap::new()));
    pub static ref REFRESH_TOKEN_LIST: Arc<Mutex<u64>> =
        Arc::new(Mutex::new(0));
    pub static ref REPLY_ACTION: Arc<Mutex<HashMap<i64, Action>>> =
        Arc::new(Mutex::new(HashMap::new()));
}

impl TOKEN_LIST {
    pub fn get_result(&self, token_name: &str) -> anyhow::Result<Arc<TokenInfo>> {
        Ok(self
            .lock()
            .map_err(|e| {
                anyhow!(
                    "Poison lock in TokenInfo get for {} --> {}",
                    token_name,
                    e.to_string()
                )
            })?
            .get(token_name)
            .ok_or(anyhow!("Couldn't access TOKEN for user {}", token_name))?
            .clone())
    }
    pub async fn refresh(&self) -> anyhow::Result<()> {
        let time_now = Utc::now().timestamp() as u64;

        let time_gap: u64 = time_now - *REFRESH_TOKEN_LIST.lock().unwrap();
        if time_gap > REFRESH_TOKEN_LIST_LIMIT {
            *REFRESH_TOKEN_LIST.lock().unwrap() = time_now;
            update_token_list().await?;
        } else {
            return Err(anyhow!(
                "Token refresh time not exceeded: {} seconds remaining",
                REFRESH_TOKEN_LIST_LIMIT - time_gap
            ));
        };
        Ok(())
    }
}
impl REFRESH_RATE_PER_USER {
    pub fn get_result(&self, user_id: UserId) -> anyhow::Result<u64> {
        Ok(*self
            .lock()
            .map_err(|e| {
                anyhow!(
                    "Poison lock in REFRESH_RATE_PER_USER get for user{} --> {}",
                    user_id.0,
                    e.to_string()
                )
            })?
            .get(&user_id)
            .unwrap_or(&0_u64))
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
pub const REFRESH_TOKEN_LIST_LIMIT: u64 = 5;
