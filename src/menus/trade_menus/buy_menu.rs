use crate::get_wallet_from_title_and_buttons;
use crate::globals::*;
use crate::handlers::constants_callbacks::*;
use crate::traits::{InlineKeyBoardHandler, OmnixString, PKeyHandler};
use crate::types::hyperliquid_client::{Balance, HyperLiquidNetwork};
use crate::utils::keys_and_addresses::*;
use crate::AddressForBot;
use crate::{
    get_main_and_faq_banner, get_refresh_button, hyperliquid_api::*, modify_message_with_buttons,
    send_unexpected_error,
};
use anyhow::Result;
use hyperliquid_rust_sdk::ClientTrigger;
use hyperliquid_rust_sdk::{
    BaseUrl, ClientCancelRequest, ClientLimit, ClientOrder, ClientOrderRequest, ExchangeClient,
    ExchangeDataStatus, ExchangeResponseStatus,
};

use ethers::types::{Address, U256};
use ethers::utils::{format_units, parse_units};
use std::str::FromStr;
use teloxide::prelude::*;
use teloxide::types::*;

pub async fn buy_menu(
    user_id: UserId,
    token_name: Option<String>,
) -> anyhow::Result<(String, InlineKeyboardMarkup)> {
    let desired_token = match token_name {
        Some(token) => token,
        _ => "WAGMI".to_owned(),
    };

    let client = HyperLiquidNetwork::get_client();

    let user_pks = WALLETS_PKEY.get_result(user_id)?;
    let user_addresses = vec_3_p_keys_to_address(&user_pks);
    let balances_raw = client
        .fetch_spot_balance_for_addresses(&user_addresses)
        .await?;
    let mut balances_usdc = Vec::new();
    for balances in &balances_raw {
        let usdc_balance: Vec<&Balance> = balances.iter().filter(|x| x.coin == "USDC").collect();
        balances_usdc.push(usdc_balance.into_iter().next())
    }

    let text = format_buy_message(balances_usdc);

    let inline_keyboard = get_buy_menu_keyboard(&desired_token);

    Ok((text, inline_keyboard))
}

pub async fn buy_menu_from_keyboard(
    user: &User,
    keyboard: InlineKeyboardMarkup,
) -> Result<(String, InlineKeyboardMarkup)> {
    let mut mutable_keyboard = keyboard.clone();
    let buy_object = get_values_from_buy_markup(keyboard)?; //TODO
                                                            // let BuyMenuObject {
                                                            //     chain,
                                                            //     wallet_index,
                                                            //     amount,
                                                            //     token,
                                                            //     slippage: _,
                                                            // } = buy_object.clone();

    let mut desired_symbol = "???".to_string();

    // if let Some(params) = create_kyber_object(user, buy_object).await {

    if let Some(last_line) = mutable_keyboard.inline_keyboard.last_mut() {
        *last_line = vec![InlineKeyboardButton::callback(
            &format!("Buy {desired_symbol}"),
            &format!("{REPLY_ACT}_{BUY_MENU}_{BUY}"),
        )]
    }
    let text = format_buy_message(vec![None, None, None]);

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

#[derive(Debug, Clone)]
pub struct BuyMenuObject {
    pub chain: String,
    pub wallet_index: usize,
    pub amount: f64,
    pub token: Address,
    pub slippage: Option<f64>,
}

pub fn get_values_from_buy_markup(
    keyboard: InlineKeyboardMarkup,
) -> anyhow::Result<(usize, ClientOrderRequest)> {
    let amount_string = keyboard.get_result_from_checked_callback_fct(AMOUNT_PLAIN)?;
    let sz: f64 = amount_string.clean_and_parse_to_float()?;
    let string_wallet = keyboard.get_result_from_checked_callback_fct(WALLET)?;
    let wallet_index = string_wallet.clean_and_parse_to_usize()? - 1;

    let limit_px = keyboard
        .get_result_from_callback_fct(SET_AMOUNT_PLAIN)?
        .clean_and_parse_to_float()
        .unwrap_or(0.0);

    let token_str = keyboard.get_result_from_callback_fct(SET_TOKEN_NAME)?;
    let token = TOKEN_LIST.get_result(token_str)?;
    let asset = token
        .usdc_pair_name()
        .ok_or(anyhow::anyhow!("Wrong pair"))?;
    let order_type = match limit_px {
        x if x != 0.0 => ClientOrder::Limit(ClientLimit {
            tif: "Gtc".to_string(),
        }),
        _ => ClientOrder::Trigger(ClientTrigger {
            is_market: true,
            tpsl: "tp".to_owned(),
            trigger_px: 0.0,
        }),
    };

    Ok((
        wallet_index,
        ClientOrderRequest {
            asset,
            is_buy: true,
            reduce_only: false,
            limit_px,
            sz,
            cloid: None,
            order_type,
        },
    ))
}
fn format_buy_message(bal_raw: Vec<Option<&Balance>>) -> String {
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

pub fn get_buy_menu_keyboard(desired_token: &str) -> InlineKeyboardMarkup {
    let (wallet_title, wallet_buttons) = get_wallet_from_title_and_buttons(BUY_MENU);

    InlineKeyboardMarkup::new(vec![
        get_main_and_faq_banner(),
        vec![get_refresh_button(BUY_MENU)],
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
            &format!(""),
            &format!("{REPLY_ACT}_{BUY_MENU}_{SET_AMOUNT_PLAIN}"),
        )],
        vec![InlineKeyboardButton::callback(
            "SEND TX",
            &format!("{REPLY_ACT}_{BUY_MENU}_{BUY}"),
        )],
    ])
}

use crate::limit_sell_menu_from_keyboard;
pub async fn spawn_limit_sell_menu_from_keyboard(
    bot: &Bot,
    user: &User,
    msg_id: MessageId,
    keyboard: InlineKeyboardMarkup,
) {
    match limit_sell_menu_from_keyboard(user, keyboard).await {
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
