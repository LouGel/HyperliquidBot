use crate::{globals::*, DBTable, Login};

use crate::utils::hash::hash_data;
use anyhow::Result;

pub fn is_passwd_set(user_id_number: i64) -> bool {
    PASSWD.lock().unwrap().contains_key(&user_id_number)
}

pub async fn set_passwd(user_id_number: i64, passwd: String) -> Result<()> {
    let pool = get_pool();
    let passwd_opt = {
        let passwd_map = PASSWD.lock().unwrap();
        passwd_map.get(&user_id_number).cloned()
    };
    let hashed_passwd = hash_data(passwd);

    match passwd_opt {
        None => {
            DBTable::insert_table::<Login>(
                &pool,
                "userid,pass",
                &format!("{},'{}'", user_id_number, hashed_passwd),
            )
            .await?
        }
        Some(_) => {
            DBTable::update_table::<Login>(
                &pool,
                user_id_number,
                &format!("pass = '{hashed_passwd}'"),
            )
            .await?
        }
    };
    {
        let mut passwd_map = PASSWD.lock().unwrap();
        passwd_map.insert(user_id_number, hashed_passwd);
    }
    Ok(())
}
pub fn is_correct_password(user_id_number: i64, given_password: String) -> bool {
    let correct_password = {
        let passwd_map = PASSWD.lock().unwrap();
        passwd_map.get(&user_id_number).cloned()
    };
    if let Some(correct_password_str) = correct_password {
        hash_data(given_password) == correct_password_str
    } else {
        true
    }
}
