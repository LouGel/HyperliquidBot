use crate::hyperliquid_api::{SecretKeyLimitOrder, SignOrderMsgResponse};
use crate::{
    AddressForBot, CancelOrderStep, InlineKeyBoardHandler, PKeyHandler, ReplyAction, WALLETS_PKEY,
};
use anyhow::anyhow;

use ethers::types::transaction::eip712::TypedData;

use crate::handlers::constants_callbacks::*;
use teloxide::types::UserId;

use reqwest::Client;
use serde::Serialize;
#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CancelOrderSignRequest {
    pub chain_id: String,
    pub maker: String,
    pub order_ids: Vec<i32>, // Order IDs to cancel
}
impl CancelOrderSignRequest {
    pub async fn get(&self) -> anyhow::Result<TypedData> {
        let client = Client::new();

        let res = client
            .post("https://limit-order.kyberswap.com/write/api/v1/orders/cancel-sign")
            .header("User-Agent", "curl/7.64.1")
            .header("x-client-id", "HyperliquidX")
            .json(self)
            .send()
            .await
            .map_err(|e| anyhow!("Error in get signature {}", e.to_string()))?;

        if !res.status().is_success() {
            let status = res.status();
            // let body = res.text().await.unwrap_or_default();
            return Err(anyhow!(
                "Wrong status  for fetch orders status :{}",
                status.as_u16()
            ));
        }
        let body = res.text().await.expect("Failed to read response text");
        let api_response: SignOrderMsgResponse = serde_json::from_str(&body)
            .map_err(|e| anyhow!("Failed to parse response: {} with body {}", e, body))?;
        Ok(api_response.data)
    }
    pub fn create_order_request(self, signature: String) -> CancelOrderRequest {
        let CancelOrderSignRequest {
            chain_id,
            maker,
            order_ids,
        } = self;
        CancelOrderRequest {
            chain_id,
            maker,
            order_ids,
            signature,
        }
    }
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CancelOrderRequest {
    pub chain_id: String,
    pub maker: String,
    pub order_ids: Vec<i32>,
    pub signature: String, // Signed message
}
use serde_json::to_string;

impl CancelOrderRequest {
    pub async fn post(&self) -> anyhow::Result<()> {
        let client = Client::new();

        let res = client
            .post("https://limit-order.kyberswap.com/write/api/v1/orders/cancel")
            .json(&self)
            .header("User-Agent", "curl/7.64.1")
            .header("x-client-id", "HyperliquidX")
            .send()
            .await
            .map_err(|e| anyhow!("Error in get signature {}", e.to_string()))?;
        if !res.status().is_success() {
            let status = res.status();
            // let body = res.text().await.unwrap_or_default();
            return Err(anyhow!(
                "Wrong status  for fetch orders status :{}",
                status.as_u16()
            ));
        }
        Ok(())
    }
}

pub async fn cancel_order(user_id: UserId, action: ReplyAction) -> anyhow::Result<()> {
    // Directly pattern match on the action to destructure it for CancelOrder
    if let ReplyAction::CancelOrder(CancelOrderStep::AnswerOrderNo(order_no), message_from) = action
    {
        let p_key = WALLETS_PKEY.get_pk_for_index(user_id, order_no.wallet_index)?;
        todo!("Cancel order");
        Ok(())
    } else {
        Err(anyhow!("Invalid action for cancel_order: {:?}", action))
    }
}
