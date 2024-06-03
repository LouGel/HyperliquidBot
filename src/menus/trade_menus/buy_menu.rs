use crate::traits::{InlineKeyBoardHandler, OmnixString, PKeyHandler};
use crate::{get_main_and_faq_banner,  get_refresh_button, modify_message_with_buttons, send_unexpected_error, hyperliquid_api::*};
use crate::globals::*;
use crate::AddressForBot;
use ethers::types::{Address, U256};
use anyhow::Result;
use ethers::utils::{format_units, parse_units};
use std::str::FromStr;
use teloxide::prelude::*;
use teloxide::types::*;
use crate::handlers::constants_callbacks::*;





pub async fn buy_menu(
    user_id: UserId,
    token_name: String,
) -> anyhow::Result<(String, InlineKeyboardMarkup)> {

    let total_gas_price = "-";
    let desired_token = "-";
    let supplement_fee = "-";
    let text = format_buy_message(

        supplement_fee,
        desired_token,
   
    );

    let inline_keyboard = get_buy_menu_keyboard( &desired_token);

    Ok((text, inline_keyboard))
}


pub fn parse_from_native( amount: f64) -> String {
    let ret =parse_units(amount, 6).unwrap();
    ret.to_string()
}
use crate::utils::format_float;
pub async fn buy_menu_from_keyboard(

    user: &User,
    keyboard: InlineKeyboardMarkup,
) -> Result<(String, InlineKeyboardMarkup)> {
    let mut mutable_keyboard = keyboard.clone();
    let buy_object = get_values_from_buy_markup(keyboard)?; //TODO
    let BuyMenuObject {
        chain,
        wallet_index,
        amount,
        token,
        slippage: _,
    } = buy_object.clone();



    let mut total_gas_price: String = "-".to_string();
    let mut desired_symbol = "???".to_string();
    let mut supplement_fee = "-".to_string();

    // if let Some(params) = create_kyber_object(user, buy_object).await {
   
    if let Some(last_line) = mutable_keyboard.inline_keyboard.last_mut() {
        *last_line = vec![InlineKeyboardButton::callback(
            &format!("Buy {desired_symbol}"),
           &format!("{REPLY_ACT}_{BUY_MENU}_{BUY}"),
        )]
    }
    let text = format_buy_message(

        &desired_symbol,
        &supplement_fee,
  
    );

    Ok((text, mutable_keyboard))
}
pub async fn spawn_buy_menu_from_keyboard(
    bot: &Bot,
    user: &User,
    msg_id: MessageId,
    keyboard: InlineKeyboardMarkup,
) {
    match buy_menu_from_keyboard(user, keyboard).await {
        Ok((text, keyboard)) => {
            modify_message_with_buttons(bot, user, msg_id, &text, &keyboard);
        }
        Err(e) => send_unexpected_error(bot, user, e.to_string()),
    }
}
use crate::limit_buy_menu_from_keyboard;
pub async fn spawn_limit_buy_menu_from_keyboard(
    bot: &Bot,
    user: &User,
    msg_id: MessageId,
    keyboard: InlineKeyboardMarkup,
) {
    match limit_buy_menu_from_keyboard( user, keyboard).await {
        Ok((text, keyboard)) => {
            modify_message_with_buttons(bot, user, msg_id, &text, &keyboard);
        }
        Err(e) => send_unexpected_error(bot, user, e.to_string() + "in spawn limit_buy"),
    }
}
use crate::limit_sell_menu_from_keyboard;
pub async fn spawn_limit_sell_menu_from_keyboard(
    bot: &Bot,
    user: &User,
    msg_id: MessageId,
    keyboard: InlineKeyboardMarkup,
) {
    match limit_sell_menu_from_keyboard( user, keyboard).await {
        Ok((text, keyboard)) => {
            modify_message_with_buttons(bot, user, msg_id, &text, &keyboard);
        }
        Err(e) => send_unexpected_error(bot, user, e.to_string() + "in spawn limit_sell"),
    }
}
use crate::sell_menu_from_keyboard;
pub async fn spawn_sell_menu_from_keyboard(
    bot: &Bot,
    user: &User,
    msg_id: MessageId,
    keyboard: InlineKeyboardMarkup,
) {
    match sell_menu_from_keyboard(user, keyboard).await {
        Ok((text, keyboard)) => {
            modify_message_with_buttons(bot, user, msg_id, &text, &keyboard);
        }
        Err(e) => send_unexpected_error(bot, user, e.to_string()),
    }
}


#[derive(Debug, Clone)]
pub struct BuyMenuObject {
    pub chain: String,
    pub wallet_index: usize,
    pub amount: f64,
    pub token: Address,
    pub slippage: Option<f64>,
}
use crate::address;
// KyberSwapParams
pub fn get_values_from_buy_markup(keyboard: InlineKeyboardMarkup) -> anyhow::Result<BuyMenuObject> {
    let amount_string = keyboard.get_result_from_checked_callback_fct(AMOUNT_PLAIN)?;
    let amount: f64 = amount_string.clean_and_parse_to_float()?;
    let string_wallet = keyboard.get_result_from_checked_callback_fct(WALLET)?;
    let wallet_index =  string_wallet.clean_and_parse_to_usize()? -1;

    let token_str = keyboard.get_result_from_callback_fct(SET_ADDRESS)?;
    let chain = keyboard
        .get_result_from_callback_fct(CHANGE_NETWORK)?
        .to_lowercase();
    let token = address!(&token_str);
    let slippage = keyboard
        .get_value_from_callback_fct(SLIPPAGE)
        .and_then(|slip| slip.clean_and_parse_to_float().ok());

    Ok(BuyMenuObject {
        chain,
        wallet_index,
        amount,
        token,
        slippage,
    })
}


pub fn parse_slippage(opt : Option<f64>) -> u16 {
    match opt {
        None => 100,
        Some(amount) => (amount * 10000_f64) as u16
    }
}



fn format_buy_message(
  
    token_name: &str,
    supplement_fee: &str,


) -> String {
    format!(
        "<b>➕ Buy Tokens\n\
        Buy any cryptocurrency on <u>HyperLiquid</u>.</b>\n\
        <em>🔍 Trade <u>{}</u> : swaps are managed by Kyberswap, an aggregator guaranteeing you the best prices.</em>\n\n\
        Supplement fee: <u>{}</u>",
         token_name, supplement_fee,  
    )
}

// pub fn extract_underlined_values(
//     formatted_str: &str,
// ) -> Option<Vec<String>> {
//     let re = Regex::new(r"<u>(.*?)</u>").unwrap();
//     let matches: Vec<String> = re
//         .captures_iter(formatted_str)
//         .map(|cap| cap[1].to_string())
//         .collect();

//     if matches.len() > 0 {
//         Some(matches)
//     } else {
//         None
//     }
// }


use crate::{ get_wallet_from_title_and_buttons};




pub fn get_buy_menu_keyboard(desired_token : &str) -> InlineKeyboardMarkup {
    let (wallet_title, wallet_buttons) = get_wallet_from_title_and_buttons(BUY_MENU);

    InlineKeyboardMarkup::new(vec![
        get_main_and_faq_banner(),
       vec![
            get_refresh_button(BUY_MENU),
        ],
        vec![InlineKeyboardButton::callback(
            "ExpertMode ❌",
           &format!("{DYN_ACTION}_{BUY_MENU}_{TOGGLE_EXPERT}"),
        )],
        vec![wallet_title],
        wallet_buttons,
        vec![InlineKeyboardButton::callback(
            &format!("AMOUNT USD USED TO BUY"),
            DEAD_CALLBACK,
        )],
        vec![
            InlineKeyboardButton::callback("✅ 0.1",&format!("{DYN_ACTION}_{BUY_MENU}_{AMOUNT_PLAIN}_0.1")),
            InlineKeyboardButton::callback("0.2",&format!("{DYN_ACTION}_{BUY_MENU}_{AMOUNT_PLAIN}_0.2")),
            InlineKeyboardButton::callback("0.5",&format!("{DYN_ACTION}_{BUY_MENU}_{AMOUNT_PLAIN}_0.5")),
        ],
        vec![
            InlineKeyboardButton::callback("1.0",&format!("{DYN_ACTION}_{BUY_MENU}_{AMOUNT_PLAIN}_1.0")),
            InlineKeyboardButton::callback("Custom",&format!("{REPLY_ACT}_{BUY_MENU}_{AMOUNT_PLAIN}_{CUSTOM}")),
        ],
        vec![InlineKeyboardButton::callback("TOKEN ADDRESS", DEAD_CALLBACK)],
        vec![InlineKeyboardButton::callback(
            &format!("{desired_token}"),
           &format!("{REPLY_ACT}_{BUY_MENU}_{SET_ADDRESS}"),
        )],
        vec![InlineKeyboardButton::callback("Waiting for address", DEAD_CALLBACK)],
    ])
}