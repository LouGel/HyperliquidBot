use crate::traits::PKeyHandler;
use ethers::core::k256::SecretKey;
use ethers::prelude::*;
// CHECK THE ERROR IN ERRORS

pub fn vec_3_p_keys_to_address(private_keys: &Vec<SecretKey>) -> Vec<Address> {
    private_keys.into_iter().map(|x| x.to_address()).collect()
}

pub trait AddressForBot {
    fn to_full_string(&self) -> String;
    // fn to_ethers_bytes(&self) -> ethers::types::Bytes;
}
impl AddressForBot for H160 {
    fn to_full_string(&self) -> String {
        format!("{:#x}", self)
    }
    // fn to_ethers_bytes(&self) -> ethers::types::Bytes {
    //     Bytes::from(hex::decode(self.to_full_string().replace("0x", "")).unwrap())
    // }
}

pub fn generate_pks() -> Vec<SecretKey> {
    let mut keys = Vec::new();

    for _ in 0..3 {
        match SecretKey::generate_random() {
            Ok(key) => keys.push(key),
            Err(_) => error!("error generating p_key "),
        }
    }
    keys
}

pub fn is_ethereum_private_key(s: &str) -> bool {
    s.len() == 64 && s.chars().all(|c| c.is_digit(16))
}
