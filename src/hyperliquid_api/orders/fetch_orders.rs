use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Serialize)]
struct OpenOrdersRequest {
    #[serde(rename = "type")]
    request_type: String,
    user: String,
}

// pub fn fetch_open_orders(url : String, address : Address ) -> OpenOrdersResponse {

// }
