use anyhow::anyhow;
use anyhow::Result;
use ethers::{prelude::*, types::transaction::eip712::TypedData};
use ethers_core::k256::elliptic_curve::SecretKey;
use ethers_core::k256::Secp256k1;

use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct SignOrderMsgResponse {
    pub code: u32,
    pub message: String,
    pub data: TypedData,
}

#[derive(Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SignOrderMsgRequest {
    pub chain_id: String,
    pub maker_asset: String,
    pub taker_asset: String,
    pub maker: String,
    pub making_amount: String,
    pub taking_amount: String,
    pub expired_at: u64,
}

impl SignOrderMsgRequest {
    fn create_order_request(&self, salt: String, signature: String) -> CreateOrderRequest {
        let SignOrderMsgRequest {
            chain_id,
            maker_asset,
            taker_asset,
            maker,
            making_amount,
            taking_amount,
            expired_at,
        } = self.clone();
        CreateOrderRequest {
            chain_id,
            maker_asset,
            taker_asset,
            maker,
            making_amount,
            taking_amount,
            expired_at,
            salt,
            signature,
        }
    }
    //////////////////////////////////////////////////////////////////////////////////////////////////
    async fn get(&self) -> Result<(String, TypedData)> {
        todo!("get order");
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateOrderRequest {
    chain_id: String,
    maker_asset: String,
    taker_asset: String,
    maker: String,
    making_amount: String,
    taking_amount: String,
    expired_at: u64,
    salt: String,
    signature: String,
}

impl CreateOrderRequest {
    async fn post(&self) -> Result<()> {
        let client = Client::new();
        let request_body = serde_json::to_string(&self)?;
        println!("Serialized request body: {}", request_body);
        let res = client
            .post("https://limit-order.kyberswap.com/write/api/v1/orders")
            .header("User-Agent", "curl/7.64.1")
            .header("x-client-id", "HyperliquidX")
            .json(self)
            .send()
            .await?;

        if !res.status().is_success() {
            let status = res.status();
            let body = res.text().await.unwrap_or_default();
            return Err(anyhow!(
                "Wrong statatus  for fetch orders: {}, status :{:?}",
                body,
                status
            ));
        }
        println!("Res : {:?}", res);
        Ok(())
    }
}

pub trait SecretKeyLimitOrder {
    async fn sign_limit_order_message(&self, order_message: TypedData) -> Result<String>;
}

impl SecretKeyLimitOrder for SecretKey<Secp256k1> {
    async fn sign_limit_order_message(&self, order_message: TypedData) -> Result<String> {
        todo!()
    }
}
#[derive(Copy, Clone)]
pub enum OrderType {
    Buy,
    Sell,
}
async fn get_taker_amount_from_maker_and_perc(
    amount_raw_maker: String,
    token_in: String,
    token_out: String,
    chain: &str,
    percent: u16,
    order_type: OrderType,
) -> Result<U256> {
    debug!("Get taker :{amount_raw_maker}, {token_in},{token_out}, {chain}, {percent} ");
    todo!("get_taker_amount_from_maker_and_perc")
}
use crate::AddressForBot;
use crate::PKeyHandler;

use tokio::join;
pub async fn send_order(
    token_address: Address,
    making_amount_raw: U256,
    percent: u16,
    p_key: SecretKey<Secp256k1>,
    expired_at: u64,
    order_type: OrderType,
) -> Result<()> {
    todo!("send_order");
    Ok(())
}

#[tokio::test]
async fn test_get_unsigned_create_order_message() -> Result<()> {
    let order_details = &SignOrderMsgRequest {
        chain_id: "1".to_string(), // Example chain ID
        maker_asset: "0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2".to_string(),
        taker_asset: "0x6B175474E89094C44Da98b954EedeAC495271d0F".to_string(),
        maker: "0xEA340c1FB4D84D4f9B5b6455E743AEFfA730565D".to_string(),
        making_amount: "1000000000000000000".to_string(), // 1 Token in Wei for example
        taking_amount: "3000000000000000000000".to_string(), // Equivalent amount of another token
        expired_at: 1755511240,                           // Example timestamp
    };
    let p_key_str = "7abbef732572d307443b3444d8059e85e85b363f5e01db075c2ecc243507c109";
    let pk_raw = hex::decode(p_key_str)?;
    let p_key = SecretKey::<Secp256k1>::from_slice(&pk_raw)?;

    // println!("Random pk {}", p_key.to_hex_string());

    let (salt, typed_data) = order_details
        .get()
        .await
        .map_err(|_| anyhow!("Error fetching unsigned order message"))?;

    println!("salt: {}", salt);
    let signature = p_key.sign_limit_order_message(typed_data).await?;

    println!("Signature: {}", signature);

    order_details
        .create_order_request(salt, signature)
        .post()
        .await
        .map_err(|e| {
            if !e.to_string().contains("Input is invalid: signature") {
                anyhow!("Unexpected error: {}", e)
            } else {
                e
            }
        })?;

    Ok(())
}
