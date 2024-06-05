use crate::*;

use sqlx::pool::Pool;
use sqlx::Postgres;
use tokio::time::{self, Duration};

pub async fn init_omni_bot() {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    init_pool(database_url).await;
    let pool = get_pool();
    let amount_pks_table = pool.fetch_pks().await.expect("Unable to fetch pk");

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
