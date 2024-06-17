// #![allow(non_snake_case)]
use serde::{Deserialize, Serialize};
use sqlx::postgres::PgRow;
use sqlx::FromRow;

use anyhow::{anyhow, Result};
use sqlx::{Pool, Postgres};

/*
SO here db table serve as a boilerplate to interact with the database.
I wanted to be consises so <T> is taking every bddd objects (bdd.rs).
I could implement it in pool but the lifetime ('r) could be too long because we decalre the pool at the begining.
Here we can use the funcs.
*/
pub struct DBTable {}
// #[async_trait]
impl DBTable {
    pub async fn get_all_of_table<T>(pool: &Pool<Postgres>) -> Result<Vec<T>>
    where
        T: for<'r> FromRow<'r, PgRow> + Send + Sync + Unpin,
    {
        let full_name = std::any::type_name::<T>();
        let table_name = full_name.split("::").last().unwrap().to_lowercase();
        info!("table_name ==> {}", table_name);

        let query = format!("SELECT * FROM {}", table_name);

        let rows = sqlx::query_as::<_, T>(&query)
            // .bind(user_id_number)
            .fetch_all(pool)
            .await?;

        Ok(rows)
    }

    pub async fn insert_table<T>(
        pool: &Pool<Postgres>,
        infos_name: &str,
        infos_values: &str,
    ) -> Result<()>
    where
        T: for<'r> FromRow<'r, PgRow> + Send + Sync + Unpin,
    {
        let full_name = std::any::type_name::<T>();
        let table_name = full_name.split("::").last().unwrap().to_lowercase();

        let sql_statement = format!(
            "INSERT INTO {} ({}) VALUES ({})",
            table_name, infos_name, infos_values
        );
        sqlx::query(&sql_statement)
            .execute(pool)
            .await
            .map_err(|e| anyhow!("Error in inster table {}, e : {:?}", full_name, e))?;
        Ok(())
    }
    pub async fn update_table<T>(
        pool: &Pool<Postgres>,
        user_id_number: i64,
        infos_values: &str,
    ) -> Result<()>
    where
        T: for<'r> FromRow<'r, PgRow> + Send + Sync + Unpin,
    {
        let full_name = std::any::type_name::<T>();
        let table_name = full_name.split("::").last().unwrap().to_lowercase();
        info!("Nemd --> {}", table_name);
        let sql_statement = format!(
            "UPDATE {} SET {} WHERE userid = {}",
            table_name, infos_values, user_id_number
        );
        sqlx::query(&sql_statement)
            .execute(pool)
            .await
            .map_err(|e| anyhow!("Error in update table {}, e : {:?}", full_name, e))?;
        Ok(())
    }
}

#[derive(Serialize, Deserialize, FromRow, Debug)]
pub struct Pks {
    pub userid: i64,
    pub pk1: String,
    pub pk2: String,
    pub pk3: String,
}

#[derive(Serialize, Deserialize, FromRow, Debug, Clone)]

pub struct Login {
    pub userid: i64,
    pub pass: String,
}

#[derive(Serialize, Deserialize, FromRow, Debug, Clone)]
pub struct Registered {
    pub userid: Option<i64>,
    pub firstmsgid: Option<i64>,
}

// #[derive(Serialize, Deserialize, FromRow, Debug, Clone)]

// pub struct Buylimit {
//     pub userid: i64,
//     pub lastmenuid: i32,
//     pub wallet: i32,
//     pub selectamount: f64,
//     pub ercaddress: String,
//     pub limitorder: f64,
//     pub expirationhour: i32,
// }
// #[derive(FromRow, Debug, Clone)]

// pub struct Buyorders {
//     pub id: i64,
//     pub userid: i64,
//     pub wallet: i32,
//     pub symbol: String,
//     pub makerasset: String,
//     pub takerasset: String,
//     pub amountin: String,
//     pub amountout: String,
//     pub outoecimals: i32,
//     pub timed: chrono::NaiveDateTime,
//     pub salt: String,
//     pub expiration: String,
// }
// #[derive(Serialize, Deserialize, FromRow, Debug, Clone)]

// pub struct Selllimit {
//     pub userid: i64,
//     pub lastmenuid: i32,
//     pub wallet: i32,
//     pub selectamount: f64,
//     pub ercaddress: String,
//     pub limitorder: f64,
//     pub expirationhour: i32,
// }
// #[derive(FromRow, Debug, Clone)]

// pub struct Sellorders {
//     pub id: i64,
//     pub userid: i64,
//     pub wallet: i32,
//     pub symbol: String,
//     pub makerasset: String,
//     pub takerasset: String,
//     pub amountin: String,
//     pub amountout: String,
//     pub outdecimals: i32,
//     pub timed: chrono::NaiveDateTime,
//     pub salt: String,
//     pub expiration: String,
// }

// #[derive(Serialize, Deserialize, FromRow, Debug, Clone)]

// pub struct Buytokens {
//     pub userid: i64,
//     pub privatetx: bool,
//     pub failguard: bool,
//     pub frontrun: bool,
//     pub ercaddress: String,
//     pub lastmenuid: i32,
//     pub wallet: i32,
//     pub selectamount: f64,
//     pub selectslippage: f64,
//     pub selltax: i32,
//     pub buytax: i32,
//     pub gasestimation: String,
// }
// #[derive(Serialize, Deserialize, FromRow, Debug, Clone)]

// pub struct Selltokens {
//     pub userid: i64,
//     pub privatetx: bool,
//     pub failguard: bool,
//     pub frontrun: bool,
//     pub lastmenuid: i32,
//     pub wallet: i32,
//     pub selectamount: f64,
//     pub selectslippage: f64,
//     pub gasestimation: String,
// }

// pub async fn get_table_by_user_id<T>(
//     pool: &Pool<Postgres>,
//     user_id_number: i64,
//     _info_name: &str,
// ) -> Result<T>
// where
//     T: for<'r> FromRow<'r, PgRow> + Send + Sync + Unpin,
// {
//     let full_name = std::any::type_name::<T>();
//     let table_name = full_name.split("::").last().unwrap().to_lowercase();
//     info!("table_name ==> {}", table_name);

//     let query = format!(
//         "SELECT * FROM {} WHERE userid = {}",
//         table_name, user_id_number
//     );

//     let rows = sqlx::query_as::<_, T>(&query)
//         .bind(user_id_number)
//         .fetch_one(pool)
//         .await?;

//     Ok(rows)
// }
