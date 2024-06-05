use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Serialize)]
struct OpenOrdersRequest {
    #[serde(rename = "type")]
    request_type: String,
    user: String,
}

#[derive(Deserialize, Debug)]
struct OpenOrder {
    coin: String,
    limitPx: String,
    oid: u64,
    side: String,
    sz: String,
    timestamp: u64,
}

#[derive(Deserialize, Debug)]
pub struct OpenOrdersResponse {
    pub orders: Vec<OpenOrder>,
}

// pub fn fetch_open_orders(url : String, address : Address ) -> OpenOrdersResponse {

// }
