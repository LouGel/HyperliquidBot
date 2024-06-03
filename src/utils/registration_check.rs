use teloxide_core::types::UserId;

use crate::globals::CHAIN_ON;

pub fn is_registered(user_id: &UserId) -> bool {
    let id_64 = user_id.0 as i64;
    CHAIN_ON.lock().unwrap().contains_key(&id_64)
}
