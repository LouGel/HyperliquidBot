use crate::types::Tokens;
use anyhow::Result;
use async_trait::async_trait;
use ethers::core::k256::SecretKey;
// use ethers::types::Address;

#[async_trait]

pub trait PoolOperation {
    async fn is_registered(&self, user_id_number: i64) -> bool;
    async fn change_chain(&self, user_id_number: i64, chain: &str);
    async fn push_pks(&self, user_id_number: u64, pks: Vec<SecretKey>) -> Result<()>;
    async fn push_one_pks(&self, user_id_number: i64, pks: SecretKey, pk_no: u8) -> Result<()>;
    async fn fetch_pks(&self) -> Result<usize>;
    async fn get_tokens_by_chain(&self, chain_id: i32) -> Result<Vec<Tokens>>;
    async fn fetch_token_by_symbol_and_chain_id(
        &self,
        symbol: &str,
        chain_id: i32,
    ) -> Result<Option<Tokens>>;
    // async fn push_token(&self, network: &Network, token_address: Address) -> Result<String>;
    // async fn get_token_by_address_and_chain(
    //     &self,
    //     token_address: Address,
    //     chain_id: i32,
    // ) -> Result<Option<Tokens>>;
    // async fn get_token_by_symbol_and_chain(
    //     &self,
    //     symbol: String,
    //     chain_id: i32,
    // ) -> Result<Option<Tokens>>;
}
