// Define your WorkerType enum
use anyhow::anyhow;
use anyhow::Result;
use ethers::abi::Abi;
use ethers::contract::Contract;
use ethers::providers::{Http, Provider};
use ethers::types::Address;
use serde::Deserialize;
use std::env;

use ethers::prelude::SignerMiddleware;
use ethers::signers::*;
use ethers_core::k256::elliptic_curve::SecretKey;
use ethers_core::k256::Secp256k1;

use std::sync::Arc;

// use networks_infos::*;

// Rust struct for Omnix
#[derive(Debug, Clone, Deserialize)]
pub struct Omnix {
    pub token: String,
    pub staking: String,
}

// Rust struct for Links
#[derive(Debug, Clone, Deserialize)]
pub struct Links {
    pub dexscreener: String,
    pub dextools: String,
}
// #[derive(Debug, Deserialize)]
// Rust struct for Network
#[derive(Debug, Clone, Deserialize)]
pub struct Network {
    pub chain_name: String,
    pub env_var: String,
    pub chain_id: u64,
    pub block_explorer: String,
    #[serde(skip_deserializing)]
    pub web3: Option<Arc<Provider<Http>>>,
    pub native_cs: String,
    pub wrapped_native_address: Address,
    pub swap_fee_address: Option<Address>,
    #[serde(skip_deserializing)]
    pub swap_fee: Option<Contract<Provider<Http>>>,
    pub ids: String,
    pub bridge_address: Option<Address>,
    pub stargate_id: Option<u32>,
    pub bridgeable_tokens: Option<Vec<u32>>,
    pub omnix: Option<Omnix>,
    pub links: Links,
    pub default_token: String,
    pub kyber_swap_router: Option<Address>,
}

pub const SWAP_FEE_ABI: &str = "./src/utils/abis/swap_fee.json";
impl Network {
    pub fn init(&mut self) -> Result<()> {
        let abi_str = std::fs::read_to_string(SWAP_FEE_ABI)?;
        let abi = serde_json::from_str::<Abi>(&abi_str)?;
        let provider_key = env::var(self.env_var.clone())?;

        let provider = Provider::<Http>::try_from(provider_key)?;
        let provider_arc = Arc::new(provider);
        self.web3 = Some(provider_arc.clone());

        if let Some(swap_address) = self.swap_fee_address.clone() {
            self.swap_fee = Some(Contract::new(swap_address, abi, provider_arc));
        }

        Ok(())
    }
    pub fn get_provider(&self) -> anyhow::Result<Arc<Provider<Http>>> {
        let provider = self
            .web3
            .clone()
            .ok_or(anyhow!("No provider for {}", self.chain_name))?;
        Ok(provider)
    }
    pub fn create_http_signer(
        &self,
        p_key: SecretKey<Secp256k1>,
    ) -> Result<SignerMiddleware<Arc<Provider<Http>>, LocalWallet>> {
        let wallet: LocalWallet = LocalWallet::from(p_key.clone());
        let wallet = wallet.with_chain_id(self.chain_id);
        Ok(SignerMiddleware::new(self.get_provider()?, wallet))
    }
}
