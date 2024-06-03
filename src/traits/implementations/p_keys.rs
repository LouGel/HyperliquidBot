use crate::PKeyHandler;
use anyhow::{anyhow, Result};
use ethers::core::k256::ecdsa::SigningKey;
use ethers::core::k256::SecretKey;
use ethers::prelude::*;
use ethers_core::rand::thread_rng;
use ethers_core::rand::RngCore;
use openssl::symm::{Cipher, Crypter, Mode};
use std::env;
use std::str;

impl PKeyHandler for SecretKey {
    fn to_hex_string(&self) -> String {
        let secret_key_bytes = self.to_bytes();
        hex::encode(secret_key_bytes)
    }
    fn encrypt(self) -> Result<String> {
        let text = self.to_bytes();
        let key = env::var("SECRET_ENCRYPTION_KEY").expect("Tochange encrypt");
        let mut iv = [0u8; 16];
        let mut rng = thread_rng();
        rng.fill_bytes(&mut iv);

        let cipher = Cipher::aes_256_cbc();
        let mut crypter = Crypter::new(cipher, Mode::Encrypt, &key.as_bytes(), Some(&iv))?;
        crypter.pad(true);

        let mut encrypted = vec![0; text.len() + cipher.block_size()];
        let count = crypter.update(&text.to_owned(), &mut encrypted)?;
        let rest = crypter.finalize(&mut encrypted[count..])?;
        encrypted.truncate(count + rest);

        let iv_hex = hex::encode(iv);
        let encrypted_hex = hex::encode(encrypted);
        let ret = iv_hex + ":" + &encrypted_hex;

        Ok(ret)
    }
    fn decrypt(text: &str) -> Result<SecretKey> {
        let key = env::var("SECRET_ENCRYPTION_KEY").expect("Encryption key not found");
        let parts: Vec<&str> = text.split(':').collect();
        if parts.len() != 2 {
            return Err(anyhow!("Invalid input format"));
        }

        let iv = hex::decode(parts[0])?;
        let encrypted_data = hex::decode(parts[1])?;

        let cipher = Cipher::aes_256_cbc();
        let mut crypter = Crypter::new(cipher, Mode::Decrypt, &key.as_bytes(), Some(&iv))?;
        crypter.pad(true);

        let mut decrypted = vec![0; encrypted_data.len() + cipher.block_size()];
        let count = crypter.update(&encrypted_data, &mut decrypted)?;
        let rest = crypter.finalize(&mut decrypted[count..])?;
        decrypted.truncate(count + rest);
        Ok(SecretKey::from_slice(&decrypted).expect("lool"))
    }
    fn generate_random() -> Result<SecretKey> {
        let mut secret_key_bytes = [0u8; 32];
        let mut rng = thread_rng();
        rng.fill_bytes(&mut secret_key_bytes);
        match SecretKey::from_slice(&secret_key_bytes) {
            Ok(key) => Ok(key),
            Err(_) => Err(anyhow!("Decrypted data is not a valid key")),
        }
    }

    fn to_address(&self) -> Address {
        let signing_key = SigningKey::from(self);
        let wallet = Wallet::from(signing_key);
        wallet.address()
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use std::env;

//     // Setup the environment for testing with a known encryption key.
//     fn setup_test_environment() {
//         // This is just for testing; never hardcode keys in production!
//         env::set_var("SECRET_ENCRYPTION_KEY", "bcfb885510ae42b3dc96e2bf35706b00");
//     }

//     #[test]
//     fn test_encrypt_known_result() {
//         setup_test_environment();

//         // Known secret key for testing
//         let secret_key_bytes = "58ac0ca2b117fbfa18e4c964d53d028a23884113cdb08c6cc7cbbeea79d8dcc0"; // A dummy key for testing
//         let secret_key =
//             SecretKey::from_slice(&secret_key_bytes).expect("Failed to create secret key");

//         let encrypted = secret_key.encrypt().unwrap();
//         println!("Encrypted: {}", encrypted);

//         // Assert something about the encrypted result if there's a known pattern or expectation.
//         // This is difficult to do with encryption because the output should be nondeterministic due to the IV.
//         assert!(!encrypted.is_empty());
//     }

//     #[test]
//     fn test_decrypt_to_known_result() {
//         setup_test_environment();

//         // For decryption, you need an encrypted string that you know decrypts to a specific value.
//         // This example assumes you have such a string. Replace "encrypted_string_here" with your known encrypted data.
//         let known_encrypted_string = "58db85051c2dbf4eaed295b6ea28c111:3ffc6a3cdd3fce97cb362efe098c36943f348cb1386ac25365e2ef7820ed64ab37c59debdfdc57738a9aa2ea9d93ce4663ffd7687ef38fb8a98c68d90a8914cf07accdd038c1a2263d4e43abc58b0b26"; // Use a real encrypted string here
//         let expected_secret_key_bytes = [0u8; 32]; // Expected result after decryption

//         let decrypted_key = SecretKey::decrypt(known_encrypted_string).expect("Failed to decrypt");
//         let decrypted_bytes = decrypted_key.to_bytes();

//         println!("Decrypted: {:?}", decrypted_key.to_hex_string());

//         // assert_eq!(decrypted_bytes, expected_secret_key_bytes);
//     }
// }
