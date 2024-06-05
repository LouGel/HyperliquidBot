use crate::handlers::constants_callbacks::{BALANCES_MENU, SET_TOKEN_DB};

use crate::{get_faq_button, get_main_menu_button};
use crate::{
    globals::*,
    hyperliquid_api::balances::{self},
    vec_3_p_keys_to_address,
};

use anyhow::Result;
use ethers::types::Address;
use ethers_core::utils::format_units;
use teloxide::{
    types::User,
    types::{InlineKeyboardButton, InlineKeyboardMarkup},
};

pub async fn balance_menu(user: &User) -> anyhow::Result<(String, InlineKeyboardMarkup)> {
    let pks = WALLETS_PKEY.get_result(user.id)?;
    let addresses = vec_3_p_keys_to_address(&pks);
    let mut text = format!(
        "<b>🤖 Hyperliquid X
    Your balance on <u>todooo!</u></b>\n"
    );

    text += &display_full_balance(addresses).await?;
    todo!()

    // Ok((text, get_balance_keyboar()))
}
use crate::handlers::constants_callbacks::REPLY_ACT;
pub fn get_balance_keyboard(chain_on: &str) -> InlineKeyboardMarkup {
    InlineKeyboardMarkup::new(vec![
        vec![get_main_menu_button(), get_faq_button()],
        vec![InlineKeyboardButton::callback(
            "Add Token",
            &format!("{REPLY_ACT}_{SET_TOKEN_DB}"),
        )],
    ])
}
pub async fn display_balance(addresses: Vec<Address>) -> Result<String> {
    Ok("Display_balance".to_owned())
}
use crate::utils::format::format_float;
pub async fn display_full_balance(addresses: Vec<Address>) -> Result<String> {
    let mut text = String::from("");

    // Concurrently fetch Ethereum balances for all addresses
    let infos_vec: Vec<String> = Vec::new();
    for (i, infos) in infos_vec.iter().enumerate() {
        // let mut wallet_block = String::new();

        // for info in infos.iter() {
        //     if info.native_token {
        //         wallet_block = match info.usd_value {
        //             Some(val) if val > 0.0 => format!(
        //                 "\nW {} \n {} {} ({}$) {}% of wallet\n",
        //                 i + 1,
        //                 info.symbol,
        //                 format_float(info.balance_formatted.clone(), 5),
        //                 format_float(info.usd_value.unwrap_or(0.00), 1),
        //                 format_float(info.portfolio_percentage, 0)
        //             ),
        //             _ => format!("\nW {} \n {} 0  0% of wallet\n", i + 1, network.native_cs),
        //         } + &wallet_block;
        //     } else {
        //         wallet_block += &format!(
        //             "{} {} ({}$) {}% of wallet\n",
        //             info.symbol,
        //             format_float(info.balance_formatted.clone(), 5),
        //             format_float(info.usd_value.unwrap_or(0.00), 1),
        //             format_float(info.portfolio_percentage, 0)
        //         );
        //     }
        // }
        // text += &wallet_block
    }
    Ok("to_do: display_full_balance".to_owned())
}
