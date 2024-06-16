use crate::{globals::*, types::*, PKeyHandler, PoolOperation};
use anyhow::{anyhow, Result};
use async_trait::async_trait;
use ethers::core::k256::SecretKey;
use sqlx::Pool;
use sqlx::Postgres;
use std::collections::HashMap;

#[async_trait]
impl PoolOperation for Pool<Postgres> {
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
}
