use crate::display_balance;
use crate::get_main_and_faq_banner;
use crate::globals::*;
use crate::handlers::constants_callbacks::*;

use crate::{
    // hyperliquid_api::{
    //     fetch_orders_by_user_id, format_orders_for_text_msg, get_raw_erc20_balance_of_address,
    //     send_order, Order, OrderType,
    // },
    menus::get_time_now,
    traits::{InlineKeyBoardHandler, OmnixString},
    vec_3_p_keys_to_address,
    PKeyHandler,
};
use anyhow::anyhow;
use teloxide::types::{InlineKeyboardButton, InlineKeyboardMarkup, User};

pub async fn sell_limit_menu(user: &User) -> anyhow::Result<(String, InlineKeyboardMarkup)> {
    let chain_name = CHAIN_ON.get_result_for_user_id(user.id)?;

    let orders = Vec::new();
    // fetch_orders_by_user_id(user.id, crate::hyperliquid_api::OrderStatus::Active).await?;
    let pks = WALLETS_PKEY.get_result(user.id)?;
    let balances = Some(display_balance(vec_3_p_keys_to_address(&pks)).await?);
    let text = format_limit_sell_message(balances, orders).await?;
    let inline_keyboard = get_limit_sell_menu_keyboard();
    Ok((text, inline_keyboard))
}
pub async fn limit_sell_menu_from_keyboard(
    user: &User,
    keyboard: InlineKeyboardMarkup,
) -> anyhow::Result<(String, InlineKeyboardMarkup)> {
    // debug!("Keyboard :{:?}", keyboard);

    let orders = Vec::new();
    //fetch_orders_by_user_id(user.id, crate::hyperliquid_api::OrderStatus::Active).await?;
    let pks = WALLETS_PKEY.get_result(user.id)?;
    let balances = Some(display_balance(vec_3_p_keys_to_address(&pks)).await?);
    let text = format_limit_sell_message(balances, orders).await?;

    Ok((text, keyboard))
}

pub async fn format_limit_sell_message(
    balances: Option<String>,
    orders: Vec<Vec<String>>,
) -> anyhow::Result<String> {
    let balance_display = match balances {
        None => "".to_owned(),
        Some(str) => str,
    };
    Ok(format!(
        "<b>🛠 Limit Sell Order\n\
        Sell cryptocurrencies on using advanced options:\n\
        Employ the Sell Limit feature to automatically trade a portion of a coin when the token's price declines,\n\
        and customize the duration for your selling settings to remain effective!</b>\n\
        <em>🔍 Trade: swaps are managed by kyberswap, an aggregator guaranteeing you the best prices.</em>\n\n
    {}\n <u>Orders</u>:\n {}",

        balance_display,
        "lol" //format_orders_for_text_msg(orders, network, Some(OrderType::Sell))?
    ))
}
pub struct LimitSellStruct {
    pub chain: String,
    pub wallet_index: usize,
    pub percentage_x100: u16,
    pub token_symbol: String,
    pub absolute_percentage_diff_x100: u16,
    pub expired_at: u64,
}
pub fn get_values_from_limit_sell_markup(
    keyboard: InlineKeyboardMarkup,
) -> anyhow::Result<LimitSellStruct> {
    let percentage_x100 = keyboard
        .get_result_from_callback_fct(AMOUNT_PERC)?
        .parse_and_scale_percentage()?;
    let string_wallet = keyboard.get_result_from_checked_callback_fct(WALLET)?;
    let wallet_index = string_wallet.clean_and_parse_to_usize()? - 1;

    let token_symbol = keyboard.get_result_from_callback_fct(SET_TOKEN_NAME)?;
    let chain = keyboard
        .get_result_from_callback_fct(CHANGE_NETWORK)?
        .to_lowercase();
    let absolute_percentage_diff_x100 = keyboard
        .get_result_from_callback_fct(SET_POSITIVE_PERC)?
        .parse_and_scale_percentage()?;
    let duration = keyboard
        .get_result_from_callback_fct(SET_DURATION)?
        .from_str_to_sec()?;
    let expired_at = get_time_now()
        .checked_add(duration)
        .ok_or(anyhow!("Could do tha addition"))?;

    Ok(LimitSellStruct {
        chain,
        wallet_index,
        percentage_x100,
        token_symbol,
        absolute_percentage_diff_x100,
        expired_at,
    })
}
pub async fn sell_from_limit_sell_menu(
    user: &User,
    menu: InlineKeyboardMarkup,
) -> anyhow::Result<String> {
    let LimitSellStruct {
        chain,
        wallet_index,
        percentage_x100,
        token_symbol,
        absolute_percentage_diff_x100,
        expired_at,
    } = get_values_from_limit_sell_markup(menu)?;
    todo!("sell_from_limit_sell_menu");

    // let network = &NETWORK_MAP.get_result(&chain)?;
    // debug!("Wallet index : {}", wallet_index);
    // let token_address = TOKEN_BY_NAME_AND_CHAINID
    //     .get(&(token_symbol.to_owned(), network.chain_id as u32))
    //     .ok_or(anyhow!("Couldn't have access to {token_symbol}"))?
    //     .address;
    // let pk = WALLETS_PKEY.get_pk_for_index(user.id, wallet_index)?;
    // let balance =
    //     get_raw_erc20_balance_of_address(network.get_provider()?, &token_address, &pk.to_address())
    //         .await?;
    // let amount = balance * percentage_x100 / 10_000;
    // // let amount_parsed = parse_from_native(network, amount);

    // send_order(
    //     network,
    //     token_address,
    //     amount,
    //     absolute_percentage_diff_x100,
    //     pk,
    //     expired_at,
    //     OrderType::Sell,
    // )
    // .await?;
    // Ok("Order posted".to_owned())
}

use crate::get_wallet_from_title_and_buttons;

fn get_limit_sell_menu_keyboard() -> InlineKeyboardMarkup {
    let (wallet_title, wallet_buttons) = get_wallet_from_title_and_buttons(SELL_LIMIT_MENU);
    InlineKeyboardMarkup::new(vec![
        get_main_and_faq_banner(),
        vec![
            InlineKeyboardButton::callback(
                "🚮 Delete order",
                &format!("{REPLY_ACT}_{SELL_LIMIT_MENU}_{CANCEL_ORDER}",),
            ),
            InlineKeyboardButton::callback(
                "🔄 Refresh Menu",
                &format!("{REFRESH_MENU}_{SELL_LIMIT_MENU}"),
            ),
        ],
        vec![wallet_title],
        wallet_buttons,
        vec![
            InlineKeyboardButton::callback("Sell Percent", DEAD_CALLBACK),
            InlineKeyboardButton::callback("Expiration", DEAD_CALLBACK),
        ],
        vec![
            InlineKeyboardButton::callback(
                "10%",
                &format!("{REPLY_ACT}_{SELL_LIMIT_MENU}_{SET_AMOUNT_PERC}"),
            ),
            InlineKeyboardButton::callback(
                "24h",
                &format!("{REPLY_ACT}_{SELL_LIMIT_MENU}_{SET_DURATION}"),
            ),
        ],
        vec![
            InlineKeyboardButton::callback("%Limit Sell", DEAD_CALLBACK),
            InlineKeyboardButton::callback("Token no", DEAD_CALLBACK),
        ],
        vec![
            InlineKeyboardButton::callback(
                "+10%",
                &format!("{REPLY_ACT}_{SELL_LIMIT_MENU}_{SET_POSITIVE_PERC}"),
            ),
            InlineKeyboardButton::callback(
                "-",
                &format!("{REPLY_ACT}_{SELL_LIMIT_MENU}_{SET_TOKEN_NAME}"),
            ),
        ],
        vec![InlineKeyboardButton::callback(
            "SEND TX",
            &format!("{REPLY_ACT}_{SELL_LIMIT_MENU}_{SELL_LIMIT}"),
        )],
    ])
}
