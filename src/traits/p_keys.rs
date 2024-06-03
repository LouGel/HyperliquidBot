use anyhow::Result;
use ethers::core::k256::SecretKey;
use ethers::prelude::*;
use std::str;

pub trait PKeyHandler {
    fn to_hex_string(&self) -> String;
    fn encrypt(self) -> Result<String>;
    fn decrypt(text: &str) -> Result<SecretKey>;
    fn generate_random() -> Result<SecretKey>;
    fn to_address(&self) -> Address;
}
