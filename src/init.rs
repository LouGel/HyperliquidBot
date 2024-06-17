use crate::*;
use anyhow::Result;
use sqlx::pool::Pool;
use sqlx::Postgres;
use tokio::time::{self, Duration};

pub async fn init_omni_bot() {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    init_pool(database_url).await;
    let pool = get_pool();
    pool.fetch_pks().await.expect("Unable to fetch pk");

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
    update_token_list()
        .await
        .map_err(|e| error!("Error while init : {:?}", e))
        .unwrap();

    time::sleep(Duration::from_millis(1000)).await;
    info!("Bot is Initiated")
}
pub async fn update_token_list() -> Result<()> {
    let client = HyperLiquidNetwork::get_client();

    let spot_tokens = client.fetch_spot_meta().await?;
    let tokens = spot_tokens.tokens;
    let mut token_map = TOKEN_LIST.lock().unwrap();

    for token in tokens {
        let token_arc = Arc::new(token.clone());
        token_map.insert(token.name.clone(), Arc::clone(&token_arc));
        token_map.insert(token.token_id.clone(), Arc::clone(&token_arc));
        token_map.insert(token.name.to_ascii_uppercase(), Arc::clone(&token_arc));
        if let Some(pair) = token.usdc_pair_name() {
            token_map.insert(pair, token_arc);
        }
    }
    debug!("Tokens updated");
    Ok(())
}

pub async fn init_pool(database_url: String) {
    info!("DB Pool instanciation");
    let pool = Pool::<Postgres>::connect(&database_url)
        .await
        .expect("Failed to create pool");
    POOL.set(Arc::new(pool)).expect("Failed to set global pool");
}
