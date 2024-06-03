use anyhow::{anyhow, Result};
use ethers::types::TxHash;
use hex::*;
use std::str::FromStr;
use url::Url;
pub trait TxLink {
    fn to_tx_link(self) -> Result<Url>;
}

impl TxLink for TxHash {
    fn to_tx_link(self) -> Result<Url> {
        let block_explorer = "Hyperliquid";

        let tx_str: String = self.encode_hex();
        let url_string = format!("https://{}/tx/0x{}", block_explorer, tx_str);

        Url::from_str(&url_string).map_err(|_e| anyhow!("Failed to parse {}  as url", url_string))
    }
}
