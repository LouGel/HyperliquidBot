use crate::AddressForBot;
use crate::{globals::*, types::*, PKeyHandler, PoolOperation};
use anyhow::{anyhow, Result};
use async_trait::async_trait;
use ethers::core::k256::SecretKey;
use ethers::types::Address;
use ring::error;
use sqlx::Pool;
use sqlx::Postgres;
use std::collections::HashMap;
use tokio::join;

#[async_trait]
impl PoolOperation for Pool<Postgres> {
    //////////////////////////////////////////////////////
    // async fn is_registered(&self, user_id_number: i64) -> bool {
    //     let exists = sqlx::query!(
    //         "SELECT EXISTS(SELECT 1 FROM registered WHERE userid = $1)",
    //         user_id_number
    //     )
    //     .fetch_one(self)
    //     .await
    //     .expect("error in is registered")
    //     .exists
    //     .unwrap_or(false);
    //     exists
    // }

    //////////////////////////////////////////////////////

    async fn fetch_pks(&self) -> Result<usize> {
        let mut buffer: HashMap<i64, Vec<SecretKey>> = HashMap::new();

        let pks: Vec<Pks> = DBTable::get_all_of_table::<Pks>(&self).await?;
        //debug!("Data fetched : {:#?}", pks);

        pks.iter().for_each(|pk| {
            let pk1 = SecretKey::decrypt(&pk.pk1).unwrap();
            let pk2 = SecretKey::decrypt(&pk.pk2).unwrap();
            let pk3 = SecretKey::decrypt(&pk.pk3).unwrap();
            let user_id_number = pk.userid as i64;
            buffer.insert(user_id_number, vec![pk1, pk2, pk3]);
        });
        let mut pk_map = WALLETS_PKEY.lock().map_err(|e| anyhow!(e.to_string()))?;
        *pk_map = buffer;
        Ok(pks.len())
    }
    //////////////////////////////////////////////////////

    async fn push_pks(&self, user_id_number: u64, pks: Vec<SecretKey>) -> Result<()> {
        debug!("IN PUSH {} PKS", pks.len());
        let encrypted: Vec<String> = pks
            .iter()
            .map(|x| x.clone().encrypt().expect("Couldn encrypt pk"))
            .collect();

        let values = format!(
            "{}, '{}', '{}', '{}'",
            user_id_number, encrypted[0], encrypted[1], encrypted[2]
        );
        debug!("Enciphered pks ==> {:#?}", encrypted);
        DBTable::insert_table::<Pks>(self, "userid,pk1,pk2,pk3", &values)
            .await
            .map_err(|e| {
                error!("{:#?}", e);
                anyhow!("DB error push pkeys {:?}", e)
            })?;
        {
            let mut pk_map = WALLETS_PKEY.lock().unwrap_or_else(|e| e.into_inner());
            pk_map.insert(user_id_number as i64, pks.clone());
        }
        Ok(())
    }

    async fn push_one_pks(&self, user_id_number: i64, pk: SecretKey, pk_no: u8) -> Result<()> {
        let encrypted = pk.clone().encrypt()?;

        let values = format!("pk{pk_no}='{encrypted}'",);
        debug!("Enciphered pks ==> {:#?}", encrypted);
        DBTable::update_table::<Pks>(self, user_id_number as i64, &values)
            .await
            .map_err(|e| {
                error!("{:#?}", e);
                anyhow!("DB error push pkeys {:?}", e)
            })?;
        {
            let mut pk_map = WALLETS_PKEY.lock().unwrap_or_else(|e| e.into_inner());
            let mut pks = pk_map
                .get(&user_id_number)
                .cloned()
                .ok_or(anyhow!("Critical error access to pk in push_one_pk"))?;
            pks[pk_no as usize - 1] = pk;
            pk_map.insert(user_id_number as i64, pks);
        }
        Ok(())
    }

    // pub struct Tokens {
    // pub tokenaddress: String,
    // pub symbol: String,
    // pub decimals: i32,
    // pub chainid: i32,
    // }
    // async fn push_token(&self, network: &Network, token_address: Address) -> Result<String> {
    //     let chain_id = network.chain_id as i32;
    //     let provider = network.web3.clone().unwrap();
    //     let (decimals, symbol_ret) = join!(
    //         get_decimals(token_address.clone(), &provider),
    //         get_symbol(token_address, &provider)
    //     );
    //     let mut symbol = symbol_ret?;
    //     // Checking if it's in the mapping
    //     if let Some(token) = TOKEN_BY_NAME_AND_CHAINID.get(&(symbol.clone(), chain_id as u32)) {
    //         if token.address == token_address {
    //             return Err(anyhow!(
    //                 "Token {} with address {:?} already exist in db",
    //                 symbol,
    //                 token_address
    //             ));
    //         }
    //         // *
    //         symbol = symbol + &token_address.to_string();
    //     } else if let Some(token) =
    //         self // Checking if it's in the database
    //             .fetch_token_by_symbol_and_chain_id(&symbol, chain_id)
    //             .await?
    //     {
    //         let address_of_fetch_token: Address = token.tokenaddress.parse()?;
    //         if address_of_fetch_token == token_address {
    //             return Err(anyhow!(
    //                 "Token {} with address {:?} already exist in db",
    //                 symbol,
    //                 token_address
    //             ));
    //         }
    //         // * We add a small address on the sign of the token
    //         symbol = symbol + &token_address.to_string();
    //     };
    //     let chain_id = network.chain_id;
    //     let token_address_str = token_address.to_full_string().to_lowercase();
    //     let info_name = "tokenaddress,symbol,decimals,chainid";
    //     let values = format!(
    //         "'{}', '{}', {}, {}",
    //         token_address_str, symbol, decimals?, chain_id
    //     );

    //     DBTable::insert_table::<Tokens>(self, info_name, &values)
    //         .await
    //         .map_err(|e| anyhow!("DB error push pkeys :{:?}", e))?;
    //     info!("Token pushed [\n{}\n{}\n]", info_name, values);
    //     Ok(symbol)
    // }
    // async fn fetch_token_by_symbol_and_chain_id(
    //     &self,
    //     symbol: &str,
    //     chain_id: i32,
    // ) -> Result<Option<Tokens>> {
    //     let query = "SELECT * FROM tokens WHERE symbol = $1 AND chainid = $2";
    //     let rows = sqlx::query_as::<_, Tokens>(&query)
    //         .bind(symbol)
    //         .bind(chain_id)
    //         .fetch_all(self)
    //         .await?;

    //     if rows.len() > 1 {
    //         Err(anyhow!("There is {} in tokens by symbol a", rows.len()))
    //     } else {
    //         Ok(rows.get(0).cloned())
    //     }
    // }
    // async fn get_tokens_by_chain(&self, chain_id: i32) -> Result<Vec<Tokens>> {
    //     let query = "SELECT * FROM tokens WHERE chainid = $1";
    //     let rows = sqlx::query_as::<_, Tokens>(&query)
    //         .bind(chain_id)
    //         .fetch_all(self)
    //         .await?;
    //     Ok(rows)
    // }

    // async fn get_token_by_address_and_chain(
    //     &self,
    //     token_address: Address,
    //     chain_id: i32,
    // ) -> Result<Option<Tokens>> {
    //     let query = "SELECT * FROM tokens WHERE tokenaddress = $1 AND chainid = $2";
    //     let token_str = token_address.to_full_string().to_lowercase();
    //     let rows = sqlx::query_as::<_, Tokens>(&query)
    //         .bind(token_str)
    //         .bind(chain_id)
    //         .fetch_all(self)
    //         .await?;

    //     if rows.len() > 1 {
    //         Err(anyhow!("There is {} in tokens by address a", rows.len()))
    //     } else {
    //         Ok(rows.get(0).cloned())
    //     }
    // }

    // async fn get_token_by_symbol_and_chain(
    //     &self,
    //     symbol: String,
    //     chain_id: i32,
    // ) -> Result<Option<Tokens>> {
    //     let query = "SELECT * FROM tokens WHERE symbol = $1 AND chainid = $2";
    //     let rows = sqlx::query_as::<_, Tokens>(&query)
    //         .bind(symbol)
    //         .bind(chain_id)
    //         .fetch_all(self)
    //         .await?;

    //     if rows.len() > 1 {
    //         Err(anyhow!("There is {} in tokens by symbol a", rows.len()))
    //     } else {
    //         Ok(rows.get(0).cloned())
    //     }
    // }
}
