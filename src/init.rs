use crate::*;

use sqlx::pool::Pool;
use sqlx::Postgres;
use tokio::time::{self, Duration};

pub async fn init_omni_bot() {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    init_pool(database_url).await;
    let pool = get_pool();
    let amount_pks_table = pool.fetch_pks().await.expect("Unable to fetch pk");

    let amount_chain_table = {
        let mut chain = CHAIN_ON.lock().unwrap();
        let basics_table: Vec<Basics> = DBTable::get_all_of_table::<Basics>(&pool).await.unwrap();
        basics_table.iter().for_each(|x| {
            let chain_name = x.chain.clone();
            debug!("Chain name in init {}", chain_name);
            chain.insert(x.userid, chain_name);
        });
        let basic_len = basics_table.len();
        info!("Fetched {} chains table", basic_len);
        basics_table.len()
    };

    assert_eq!(
        amount_chain_table, amount_pks_table,
        "Chain table amount({}) and amount pks table({}) not equal",
        amount_chain_table, amount_pks_table
    );

    {
        let mut referrals = REFERRAL.lock().unwrap();
        let referral_table: Vec<Referrals> =
            DBTable::get_all_of_table::<Referrals>(&pool).await.unwrap();
        referral_table.iter().for_each(|x| {
            referrals.insert(x.username.to_owned(), x.clone());
        });
        info!("Fetched {} refferal table", referral_table.len());
    }

    {
        let mut passwd_map = PASSWD.lock().unwrap();
        let login_tables: Vec<Login> = DBTable::get_all_of_table::<Login>(&pool)
            .await
            .expect("Error in getting passwds : ");
        login_tables.iter().for_each(|x| {
            passwd_map.insert(x.userid as i64, x.pass.clone());
        });

        info!("Fetched {} login table", login_tables.len());
    }

    {
        let mut referred = REFERRED.lock().unwrap();
        let referred_table: Vec<Referred> =
            DBTable::get_all_of_table::<Referred>(&pool).await.unwrap();
        referred_table.iter().for_each(|x| {
            referred.insert(x.userid, x.refaddress.to_owned());
        });
        info!("Fetched {} reffered table", referred_table.len());
    }

    time::sleep(Duration::from_millis(1000)).await;
    info!("Bot is Initiated")
}

pub async fn init_pool(database_url: String) {
    info!("DB Pool instanciation");
    let pool = Pool::<Postgres>::connect(&database_url)
        .await
        .expect("Failed to create pool");
    POOL.set(Arc::new(pool)).expect("Failed to set global pool");
}
