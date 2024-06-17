use crate::globals::*;

use crate::traits::PoolOperation;
use crate::types::*;
use crate::utils::*;
use teloxide::prelude::*;

pub async fn create_user(msg: &Message) -> Option<()> {
    let pool = get_pool();
    let user_id = msg.from().unwrap().id;
    let user_id_number = user_id.0 as i64;
    info!("Creating user {}", user_id_number);
    if is_registered(&user_id) {
        return None;
    }
    //// Register
    if let Err(e) = DBTable::insert_table::<Registered>(
        &pool,
        "userid,firstmsgid",
        &format!("{},{}", user_id_number, msg.id),
    )
    .await
    {
        error!("Error in start : {:?} \n for user {}", e, user_id_number);
        return None;
    }

    let pks = generate_pks();

    if let Err(e) = pool.push_pks(user_id_number as u64, pks).await {
        error!("Error in start : {:?} \n for user {}", e, user_id_number);
        return None;
    }

    Some(())
}
