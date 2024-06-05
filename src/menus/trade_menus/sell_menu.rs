use crate::handlers::constants_callbacks::*;

use crate::globals::*;
use crate::traits::{InlineKeyBoardHandler, OmnixString, PKeyHandler};
use crate::{display_balance, get_main_and_faq_banner, get_refresh_button, hyperliquid_api::*};
use crate::{vec_3_p_keys_to_address, AddressForBot};
use anyhow::anyhow;
use anyhow::Result;
use ethers::types::U256;
use ethers::utils::{format_units, parse_units};
use teloxide::types::{InlineKeyboardButton, InlineKeyboardMarkup, User};

pub async fn sell_menu(user: &User) -> anyhow::Result<(String, InlineKeyboardMarkup)> {
    let total_gas_price = "-";

    let supplement_fee = "-".to_owned();
    // get_address_from_receiver(UserId(user_id), receiver)
    let pks = WALLETS_PKEY.get_result(user.id)?;

    let text = format_sell_message(
        &supplement_fee,
        Some(display_balance(vec_3_p_keys_to_address(&pks)).await?),
        total_gas_price,
    );

    let inline_keyboard = get_sell_menu_keyboard();

    Ok((text, inline_keyboard))
}
#[derive(Debug, Clone)]
pub struct SellMenuObject {
    pub chain: String,
    pub wallet_index: usize,
    pub amount: f64,
    pub token: String,
    pub slippage: Option<f64>,
}
use crate::utils::format_float;
pub async fn sell_menu_from_keyboard(
    user: &User,
    keyboard: InlineKeyboardMarkup,
) -> Result<(String, InlineKeyboardMarkup)> {
    let mut mutable_keyboard = keyboard.clone();
    let sell_object = get_values_from_sell_markup(keyboard).unwrap(); //TODO
    let SellMenuObject {
        chain,
        wallet_index,
        amount,
        token,
        slippage: _,
    } = sell_object.clone();

    let pks = WALLETS_PKEY.get_result(user.id)?;
    let mut total_gas_price: String = "-".to_string();
    let mut supplement_fee = "-".to_string();
    let desired_symbol = token;
    // let token_info = TOKEN_BY_NAME_AND_CHAINID
    //     .get(&(desired_symbol.clone(), network.chain_id as u32))
    //     .cloned()
    //     .unwrap();
    // if let Some(params) = create_kyber_object(user, sell_object).await {

    let text = format_sell_message(
        &supplement_fee,
        Some(display_balance(vec_3_p_keys_to_address(&pks)).await?),
        &total_gas_price,
    );

    Ok((text, mutable_keyboard))
}

pub fn get_values_from_sell_markup(keyboard: InlineKeyboardMarkup) -> Result<SellMenuObject> {
    let amount_string = keyboard.get_result_from_checked_callback_fct(AMOUNT_PERC)?;
    let amount: f64 = amount_string.clean_and_parse_to_float()?;

    let string_wallet = keyboard.get_result_from_checked_callback_fct(WALLET)?;
    let wallet_index = string_wallet.clean_and_parse_to_usize()? - 1;

    let token = keyboard.get_result_from_callback_fct(SET_TOKEN_NAME)?;
    let chain = keyboard
        .get_result_from_callback_fct(CHANGE_NETWORK)?
        .to_lowercase();

    let slippage = keyboard
        .get_value_from_callback_fct(SLIPPAGE)
        .and_then(|slip| slip.clean_and_parse_to_float().ok());

    Ok(SellMenuObject {
        chain,
        wallet_index,
        amount,
        token,
        slippage,
    })
}

fn format_sell_message(
    supplement_fee: &str,
    array_of_balance: Option<String>,
    total_gas_price: &str,
) -> String {
    let intro = format!(
        "<b>➖ Sell Tokens\n\
Sell any cryptocurrency on your balance on the HyperLiquid.\n\
To associate a token with a number, go to Trade > Balances, then associate the token address with the number of your choice.</b>\n\
<em>🔍 Trade: swaps are managed by Kyberswap, an aggregator guaranteeing you the best prices.</em>\n\n" );

    let outro = format!(
        "
<i>Gas fee estimation: <u>{}</u></i>\n\
Supplement fee: <u>{}</u>",
        total_gas_price, supplement_fee
    );
    match array_of_balance {
        Some(text) => format!("{}{}{}", intro, text, outro),
        None => format!("{}{}{}", intro, "Balances : -\n", outro),
    }
}
use crate::get_wallet_from_title_and_buttons;

pub fn get_sell_menu_keyboard() -> InlineKeyboardMarkup {
    let (wallet_title, wallet_buttons) = get_wallet_from_title_and_buttons(SELL_MENU);
    InlineKeyboardMarkup::new(vec![
        get_main_and_faq_banner(),
        vec![get_refresh_button(SELL_MENU)],
        vec![InlineKeyboardButton::callback(
            "ExpertMode ❌",
            &format!("{DYN_ACTION}_{SELL_MENU}_{TOGGLE_EXPERT}"),
        )],
        vec![wallet_title],
        wallet_buttons,
        vec![InlineKeyboardButton::callback(
            &format!("AMOUNT TO SELL"),
            DEAD_CALLBACK,
        )],
        vec![
            InlineKeyboardButton::callback(
                "✅ 10.0%",
                &format!("{DYN_ACTION}_{SELL_MENU}_{AMOUNT_PERC}_10.0"),
            ),
            InlineKeyboardButton::callback(
                "25.0%",
                &format!("{DYN_ACTION}_{SELL_MENU}_{AMOUNT_PERC}_25.0"),
            ),
            InlineKeyboardButton::callback(
                "50.0%",
                &format!("{DYN_ACTION}_{SELL_MENU}_{AMOUNT_PERC}_50.0"),
            ),
        ],
        vec![
            InlineKeyboardButton::callback(
                "75.0%",
                &format!("{DYN_ACTION}_{SELL_MENU}_{AMOUNT_PERC}_75.0"),
            ),
            InlineKeyboardButton::callback(
                "100.0%",
                &format!("{DYN_ACTION}_{SELL_MENU}_{AMOUNT_PERC}_100.0"),
            ),
            InlineKeyboardButton::callback(
                "Custom%",
                &format!("{REPLY_ACT}_{SELL_MENU}_{AMOUNT_PERC}_{CUSTOM}"),
            ),
        ],
        vec![InlineKeyboardButton::callback("TOKEN NAME", DEAD_CALLBACK)],
        vec![InlineKeyboardButton::callback(
            "-",
            &format!("{REPLY_ACT}_{SELL_MENU}_{SET_TOKEN_NAME}"),
        )],
        vec![InlineKeyboardButton::callback(
            "SELL ???",
            &format!("{REPLY_ACT}_{SELL_MENU}_{SELL}"),
        )],
    ])
}
