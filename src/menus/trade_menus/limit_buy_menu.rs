use crate::get_main_and_faq_banner;
use crate::handlers::constants_callbacks::*;

use crate::traits::InlineKeyBoardHandler;
use crate::traits::OmnixString;

use crate::{
    globals::*,
    vec_3_p_keys_to_address,
    // hyperliquid_api::{
    //     fetch_orders_by_user_id, format_orders_for_text_msg, get_raw_erc20_balance_of_address,
    //     get_wrapped_and_native_amounts_for_addresses, send_order, Order, OrderType,
    // },
    PKeyHandler,
};
use anyhow::anyhow;
use ethers::types::Address;
use teloxide::{
    prelude::*,
    types::{InlineKeyboardButton, InlineKeyboardMarkup, User},
};

pub async fn limit_buy_menu(user_id: UserId) -> anyhow::Result<(String, InlineKeyboardMarkup)> {
    let chain_name = CHAIN_ON.get_result_for_user_id(user_id)?;
    let p_ks = WALLETS_PKEY.get_result(user_id)?;
    let addresses = vec_3_p_keys_to_address(&p_ks);
    let ret = Vec::new();
    //fetch_orders_by_user_id(user_id, crate::hyperliquid_api::OrderStatus::Active).await?;

    let text = format_limit_buy_message(addresses, ret).await?;

    let inline_keyboard = get_limit_buy_keyboard_from_chain_name();

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
    let ret = Vec::new();
    // fetch_orders_by_user_id(user.id, crate::hyperliquid_api::OrderStatus::Active).await?;
    let text = format_limit_buy_message(addresses, ret).await?;

    Ok((text, keyboard))
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
pub async fn format_limit_buy_message(
    addresses: Vec<Address>,
    orders: Vec<Vec<String>>,
) -> anyhow::Result<String> {
    Ok(format!(
        "<b>🛠 Limit Buy Order</b>\n
    Buy cryptocurrencies on HyperLiquid with advanced options:\n\
    Use Buy Limit to purchase when a token's price drops\n\
    and set the duration for your purchase settings to stay active! d\n\n\
    {}\n <u>Orders</u>:\n {}",
        "Native balance",
        "Format Order todo" //format_orders_for_text_msg(orders, network, Some(OrderType::Buy))?
    ))
}
pub struct LimitBuyStruct {
    pub chain: String,
    pub wallet_index: usize,
    pub percentage_x100: u16,
    pub token: Address,
    pub absolute_percentage_diff_x100: u16,
    pub expired_at: u64,
}
pub fn get_values_from_limit_buy_markup(
    keyboard: InlineKeyboardMarkup,
) -> anyhow::Result<LimitBuyStruct> {
    let percentage_x100 = keyboard
        .get_result_from_callback_fct(AMOUNT_PERC)?
        .parse_and_scale_percentage()?;
    let string_wallet = keyboard.get_result_from_checked_callback_fct(WALLET)?;
    let wallet_index = string_wallet.clean_and_parse_to_usize()? - 1;

    let token_str = keyboard.get_result_from_callback_fct(SET_ADDRESS)?;
    let chain = keyboard
        .get_result_from_callback_fct(CHANGE_NETWORK)?
        .to_lowercase();
    let token: Address = token_str.parse()?;
    let absolute_percentage_diff_x100 = keyboard
        .get_result_from_callback_fct(SET_NEGATIVE_PERC)?
        .parse_and_scale_percentage()?;
    let duration = keyboard
        .get_result_from_callback_fct(SET_DURATION)?
        .from_str_to_sec()?;
    let expired_at = get_time_now()
        .checked_add(duration)
        .ok_or(anyhow!("Could do that addition"))?;

    Ok(LimitBuyStruct {
        chain,
        wallet_index,
        percentage_x100,
        token,
        absolute_percentage_diff_x100,
        expired_at,
    })
}

//Order : get % of the ether bag -> after check the price
use crate::utils::format_float;

pub async fn buy_from_limit_buy_menu(
    user: &User,
    menu: InlineKeyboardMarkup,
) -> anyhow::Result<String> {
    let LimitBuyStruct {
        chain,
        wallet_index,
        percentage_x100,
        token,
        absolute_percentage_diff_x100,
        expired_at,
    } = get_values_from_limit_buy_markup(menu)?;
    todo!("buy from lomit menu");
    // let network = &NETWORK_MAP.get_result(&chain)?;
    // let pk = WALLETS_PKEY.get_pk_for_index(user.id, wallet_index)?;
    // let balance = get_raw_erc20_balance_of_address(
    //     network.get_provider()?,
    //     &network.wrapped_native_address,
    //     &pk.to_address(),
    // )
    // .await?;
    // let amount = balance * percentage_x100 / 10_000;
    // // let amount_parsed = parse_from_native(network, amount);

    // send_order(
    //     network,
    //     token,
    //     amount,
    //     absolute_percentage_diff_x100,
    //     pk,
    //     expired_at,
    //     OrderType::Buy,
    // )
    // .await?;
    // Ok("Order posted".to_owned())
}

use crate::get_wallet_from_title_and_buttons;

pub fn get_limit_buy_keyboard_from_chain_name() -> InlineKeyboardMarkup {
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
        vec![InlineKeyboardButton::callback(
            "📦 Wrap Native",
            &format!("{SIMPLE_MENU}_{WRAP_MENU}"),
        )],
        vec![wallet_title],
        wallet_buttons,
        vec![
            InlineKeyboardButton::callback(&format!("% of USD"), DEAD_CALLBACK),
            InlineKeyboardButton::callback("Expiration", DEAD_CALLBACK),
        ],
        vec![
            InlineKeyboardButton::callback(
                "10%",
                &format!("{REPLY_ACT}_{BUY_LIMIT_MENU}_{SET_AMOUNT_PERC}"),
            ),
            InlineKeyboardButton::callback(
                "24h",
                &format!("{REPLY_ACT}_{BUY_LIMIT_MENU}_{SET_DURATION}"),
            ),
        ],
        vec![InlineKeyboardButton::callback(
            "-% Limit Buy",
            DEAD_CALLBACK,
        )],
        vec![InlineKeyboardButton::callback(
            "-10%",
            &format!("{REPLY_ACT}_{BUY_LIMIT_MENU}_{SET_NEGATIVE_PERC}"),
        )],
        vec![InlineKeyboardButton::callback(
            "Token address",
            DEAD_CALLBACK,
        )],
        vec![InlineKeyboardButton::callback(
            "-",
            &format!("{REPLY_ACT}_{BUY_LIMIT_MENU}_{SET_ADDRESS}",),
        )],
        vec![InlineKeyboardButton::callback(
            "SEND TX",
            &format!("{REPLY_ACT}_{BUY_LIMIT_MENU}_{BUY_LIMIT}",),
        )],
    ])
}
