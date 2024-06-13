use crate::get_main_and_faq_banner;
use crate::handlers::constants_callbacks::*;

use crate::get_wallet_from_title_and_buttons;
use crate::globals::*;
use crate::traits::InlineKeyBoardHandler;

use crate::types::hyperliquid_client::{Balance, HyperLiquidNetwork};
use crate::PKeyHandler;
use crate::{
    globals::*,
    hyperliquid_api::balances::{self},
    vec_3_p_keys_to_address,
};
use crate::{modify_message_with_buttons, send_unexpected_error};
use teloxide::prelude::*;
use teloxide::types::*;

use anyhow::anyhow;
use anyhow::Result;
use ethers::types::Address;

use hyperliquid_rust_sdk::{
    BaseUrl, ClientCancelRequest, ClientLimit, ClientOrder, ClientOrderRequest, ExchangeClient,
    ExchangeDataStatus, ExchangeResponseStatus,
};

use ethers::types::U256;
use ethers::utils::{format_units, parse_units};
use std::str::FromStr;
use teloxide::prelude::*;
use teloxide::types::*;

pub async fn limit_buy_menu(user_id: UserId) -> anyhow::Result<(String, InlineKeyboardMarkup)> {
    let p_ks = WALLETS_PKEY.get_result(user_id)?;
    let addresses = vec_3_p_keys_to_address(&p_ks);
    let client = HyperLiquidNetwork::get_client();

    let balances_raw = client.fetch_spot_balance_for_addresses(&addresses).await?;
    let mut balances_usdc = Vec::new();
    for balances in &balances_raw {
        let usdc_balance: Vec<&Balance> = balances.iter().filter(|x| x.coin == "USDC").collect();
        balances_usdc.push(usdc_balance.into_iter().next())
    }

    let inline_keyboard = get_limit_buy_keyboard("WAGMI");
    let text = format_limit_buy_message(balances_usdc);
    Ok((text, inline_keyboard))
}

pub async fn limit_buy_menu_from_keyboard(
    user: &User,
    keyboard: InlineKeyboardMarkup,
) -> anyhow::Result<(String, InlineKeyboardMarkup)> {
    let chain_name: String = keyboard
        .get_result_from_callback_fct(CHANGE_NETWORK)?
        .to_lowercase();
    let p_ks = WALLETS_PKEY.get_result(user.id)?;
    let addresses = vec_3_p_keys_to_address(&p_ks);

    todo!();
    // fetch_orders_by_user_id(user.id, crate::hyperliquid_api::OrderStatus::Active).await?;
    // let text = format_limit_buy_message(vec![None]).await?;

    // Ok((text, keyboard))
}

pub async fn spawn_limit_buy_menu_from_keyboard(
    bot: &Bot,
    user: &User,
    msg_id: MessageId,
    keyboard: InlineKeyboardMarkup,
) {
    match limit_buy_menu_from_keyboard(user, keyboard).await {
        Ok((text, keyboard)) => {
            modify_message_with_buttons(bot, user, msg_id, &text, &keyboard);
        }
        Err(e) => send_unexpected_error(bot, user, e.to_string() + "in spawn limit_buy"),
    }
}

// use std::time::{SystemTime, UNIX_EPOCH};
use std::time::{SystemTime, UNIX_EPOCH};
pub fn get_time_now() -> u64 {
    let now = SystemTime::now();
    let since_the_epoch = now
        .duration_since(UNIX_EPOCH)
        .unwrap_or_else(|_| panic!("Time went backwards"));
    since_the_epoch.as_secs()
}
fn format_limit_buy_message(bal_raw: Vec<Option<&Balance>>) -> String {
    let balances: Vec<String> = bal_raw
        .iter()
        .map(|x| match x {
            Some(bal) => bal.total.clone(),
            None => "0".to_owned(),
        })
        .collect();
    format!(
        "<b>🛠WAGMI Limit Buy Order</b>\n
        Buy tokens on HyperLiquid with advanced options:
        Use Buy Limit to purchase when a token's price drops and set the duration for your purchase settings to stay active! 
        ⚠️EDIT SETTINGS WITH A PEN (✏️) EMOJI ONLY
        USDC Balance:\n
        w1 :{} $USDC\n
        w2 :{} $USDC\n
        w3 :{} $USDC
        ", balances[0], balances[1], balances[2])
}

pub fn get_limit_buy_keyboard(desired_token: &str) -> InlineKeyboardMarkup {
    let (wallet_title, wallet_buttons) = get_wallet_from_title_and_buttons(BUY_LIMIT_MENU);
    InlineKeyboardMarkup::new(vec![
        get_main_and_faq_banner(),
        vec![
            InlineKeyboardButton::callback(
                "🚮 Delete order",
                &format!("{REPLY_ACT}_{BUY_LIMIT_MENU}_{CANCEL_ORDER}"),
            ),
            InlineKeyboardButton::callback(
                "🔄 Refresh Menu",
                &format!("{REFRESH_MENU}_{BUY_LIMIT_MENU}"),
            ),
        ],
        vec![wallet_title],
        wallet_buttons,
        vec![InlineKeyboardButton::callback(
            &format!("AMOUNT USD USED TO BUY"),
            DEAD_CALLBACK,
        )],
        vec![InlineKeyboardButton::callback(
            "Amount ✏️",
            &format!("{REPLY_ACT}_{BUY_MENU}_{AMOUNT_PLAIN}_{CUSTOM}"),
        )],
        vec![InlineKeyboardButton::callback(
            &format!("TOKEN"),
            DEAD_CALLBACK,
        )],
        vec![InlineKeyboardButton::callback(
            &format!("{desired_token}"),
            &format!("{REPLY_ACT}_{BUY_MENU}_{SET_TOKEN_NAME}"),
        )],
        vec![InlineKeyboardButton::callback(
            &format!("Price"),
            DEAD_CALLBACK,
        )],
        vec![InlineKeyboardButton::callback(
            &format!("0"),
            &format!("{REPLY_ACT}_{BUY_MENU}_{SET_AMOUNT_PLAIN}"),
        )],
        vec![InlineKeyboardButton::callback(
            "SEND TX",
            &format!("{REPLY_ACT}_{BUY_MENU}_{BUY}"),
        )],
    ])
}
