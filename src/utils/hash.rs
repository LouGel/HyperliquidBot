use openssl::hash::{Hasher, MessageDigest};
use std::io::Write;

pub fn hash_data(data: String) -> String {
    let mut hasher = Hasher::new(MessageDigest::sha256()).expect("Failed to create hasher");
    hasher
        .write_all(data.as_bytes())
        .expect("Failed to write data");
    let ret = hasher.finish().expect("Failed to finish hashing").to_vec();
    hex::encode(ret)
}
