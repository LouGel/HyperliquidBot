use teloxide_core::types::UserId;

use crate::WALLETS_PKEY;

pub fn is_registered(user_id: &UserId) -> bool {
    let id_64 = user_id.0 as i64;
    WALLETS_PKEY.lock().unwrap().contains_key(&id_64)
}
