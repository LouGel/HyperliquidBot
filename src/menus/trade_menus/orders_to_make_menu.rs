use crate::display_full_balance;
use crate::display_token_balance;
use crate::get_main_and_faq_banner;
use crate::get_wallet_from_title_and_buttons;
use crate::globals::*;
use crate::handlers::constants_callbacks::*;
use crate::traits::InlineKeyBoardHandler;
use crate::types::hyperliquid_client::HyperLiquidNetwork;
use crate::vec_3_p_keys_to_address;
use crate::{modify_message_with_buttons, send_unexpected_error};
use std::time::{SystemTime, UNIX_EPOCH};
use teloxide::prelude::*;
use teloxide::types::*;
use tokio::join;

pub async fn make_orders_menu(
    user_id: UserId,
    token_name: &str,
    is_buy: bool,
    is_limit: bool,
) -> anyhow::Result<(String, InlineKeyboardMarkup)> {
    let p_ks = WALLETS_PKEY.get_result(user_id)?;
    let addresses = vec_3_p_keys_to_address(&p_ks);
    let client = HyperLiquidNetwork::get_client();

    let (price_result, text_result) =
        join!(client.clone().fetch_price_for_token(token_name), async {
            match (is_buy, is_limit) {
                (true, true) => {
                    format_limit_buy_message(addresses.clone(), "USDC".to_owned()).await
                }
                (true, false) => format_buy_message(addresses.clone(), "USDC".to_owned()).await,
                (false, true) => format_limit_sell_message(addresses.clone()).await,
                (false, false) => format_sell_message(addresses.clone()).await,
            }
        });

    let price = price_result?;
    let text = text_result?;

    let inline_keyboard = get_order_keyboard(token_name, &price, is_buy, is_limit, None, None);

    Ok((text, inline_keyboard))
}

pub async fn make_orders_menu_from_keyboard(
    user: &User,
    keyboard: InlineKeyboardMarkup,
    token_opt: Option<&str>,
) -> anyhow::Result<(String, InlineKeyboardMarkup)> {
    let (is_buy, is_limit) = keyboard.get_which_order_type()?;
    let p_ks = WALLETS_PKEY.get_result(user.id)?;
    let addresses = vec_3_p_keys_to_address(&p_ks);

    if let Some(token) = token_opt {
        let client = HyperLiquidNetwork::get_client();
        let (price_result, text_result) =
            join!(client.clone().fetch_price_for_token(token), async {
                match (is_buy, is_limit) {
                    (true, true) => {
                        format_limit_buy_message(addresses.clone(), "USDC".to_owned()).await
                    }
                    (true, false) => format_buy_message(addresses.clone(), "USDC".to_owned()).await,
                    (false, true) => format_limit_sell_message(addresses.clone()).await,
                    (false, false) => format_sell_message(addresses.clone()).await,
                }
            });

        let price = price_result?;
        let text = text_result?;
        let token_str = &format!("{token} ({price}$) ✏️");
        let mut keyboard_buf = keyboard;
        keyboard_buf.change_text_where_callback_contains(TOKEN_NAME, token_str);
        return Ok((text, keyboard_buf));
    }
    let text = match (is_buy, is_limit) {
        (true, true) => format_limit_buy_message(addresses.clone(), "USDC".to_owned()).await?,
        (true, false) => format_buy_message(addresses.clone(), "USDC".to_owned()).await?,
        (false, true) => format_limit_sell_message(addresses.clone()).await?,
        (false, false) => format_sell_message(addresses.clone()).await?,
    };
    Ok((text, keyboard))
}

pub async fn spawn_order_menu_from_keyboard(
    bot: &Bot,
    user: &User,
    msg_id: MessageId,
    keyboard: InlineKeyboardMarkup,
    token_opt: Option<&str>,
) {
    match make_orders_menu_from_keyboard(user, keyboard, token_opt).await {
        Ok((text, keyboard)) => {
            modify_message_with_buttons(bot, user, msg_id, &text, &keyboard);
        }
        Err(e) => send_unexpected_error(bot, user, e.to_string() + "in spawn order_menu"),
    }
}
use ethers::types::Address;

async fn format_limit_buy_message(
    addresses: Vec<Address>,
    token: String,
) -> anyhow::Result<String> {
    let balances = display_token_balance(addresses, token).await?;
    Ok(format!(
        "<b>🛠WAGMI Limit Buy Order</b>\n
        Buy tokens on HyperLiquid with advanced options:
        Use Buy Limit to purchase when a token's price drops and set the duration for your purchase settings to stay active! 
        ⚠️EDIT SETTINGS WITH A PEN (✏️) EMOJI ONLY
        {balances}"))
}
async fn format_buy_message(addresses: Vec<Address>, token: String) -> anyhow::Result<String> {
    let balances = display_token_balance(addresses, token).await?;
    Ok(format!(
        "<b>🛠WAGMI Buy Tokens </b>\n
        Buy tokens on HyperLiquid with market orders ! 
        ⚠️EDIT SETTINGS WITH A PEN (✏️) EMOJI ONLY
        {balances}"
    ))
}
async fn format_limit_sell_message(addresses: Vec<Address>) -> anyhow::Result<String> {
    let balances = display_full_balance(addresses).await?;
    Ok(format!(
        "<b>🛠WAGMI Limit Sell Order</b>\n
        Sell tokens on HyperLiquid with advanced options:
        Use Sell Limit to purchase when a token's price drops and set the duration for your purchase settings to stay active! 
        ⚠️EDIT SETTINGS WITH A PEN (✏️) EMOJI ONLY
        {balances}"))
}
async fn format_sell_message(addresses: Vec<Address>) -> anyhow::Result<String> {
    let balances = display_full_balance(addresses).await?;
    Ok(format!(
        "<b>🛠WAGMI Sell Tokens </b>\n
        Sell tokens on HyperLiquid with market orders ! 
        ⚠️EDIT SETTINGS WITH A PEN (✏️) EMOJI ONLY
        {balances}"
    ))
}

pub fn get_order_keyboard(
    desired_token: &str,
    price_usd: &str,
    is_buy: bool,
    is_limit: bool,
    amount: Option<String>,
    price: Option<String>,
) -> InlineKeyboardMarkup {
    let (wallet_title, wallet_buttons) = get_wallet_from_title_and_buttons(MAKE_ORDERS_MENU);
    let (main_token, buy_str) = match is_buy {
        true => ("USD".to_owned(), "Buy".to_owned()),
        false => (desired_token.to_owned(), "Sell".to_owned()),
    };
    let limit_str = match is_limit {
        true => LIMIT.to_owned(),
        false => MARKET.to_owned(),
    };
    let amount_str = match amount {
        Some(am) => am,
        None => "Amount".to_owned(),
    };
    let mut keyboard = vec![
        vec![InlineKeyboardButton::callback(
            &format!("{buy_str} {limit_str}"),
            &format!("!{MAKE_ORDERS_MENU}_{buy_str}_{limit_str}"),
        )],
        get_main_and_faq_banner(),
        vec![InlineKeyboardButton::callback(
            "🔄 Refresh Menu",
            &format!("{REFRESH_MENU}_{MAKE_ORDERS_MENU}"),
        )],
        vec![wallet_title],
        wallet_buttons,
        vec![InlineKeyboardButton::callback(
            &format!("AMOUNT {main_token} USED TO {buy_str}"),
            DEAD_CALLBACK,
        )],
        vec![InlineKeyboardButton::callback(
            &format!("{amount_str} ✏️"),
            &format!("{REPLY_ACT}_{MAKE_ORDERS_MENU}_{AMOUNT_PLAIN}"),
        )],
        vec![InlineKeyboardButton::callback(
            &format!("TOKEN"),
            DEAD_CALLBACK,
        )],
        vec![InlineKeyboardButton::callback(
            &format!("{desired_token} ({price_usd}$) ✏️"),
            &format!("{REPLY_ACT}_{MAKE_ORDERS_MENU}_{TOKEN_NAME}"),
        )],
    ];
    if is_limit {
        let price_str = match price {
            Some(am) => am,
            None => "Price".to_owned(),
        };
        keyboard.push(vec![InlineKeyboardButton::callback(
            &format!("Price"),
            DEAD_CALLBACK,
        )]);
        keyboard.push(vec![InlineKeyboardButton::callback(
            &format!("{price_str} ✏️"),
            &format!("{REPLY_ACT}_{MAKE_ORDERS_MENU}_{PRICE_WANTED}"),
        )])
    }
    keyboard.push(vec![InlineKeyboardButton::callback(
        "SEND TX",
        &format!("{REPLY_ACT}_{EXECUTE_ORDER}"),
    )]);
    InlineKeyboardMarkup::new(keyboard)
}

pub fn get_time_now() -> u64 {
    let now = SystemTime::now();
    let since_the_epoch = now
        .duration_since(UNIX_EPOCH)
        .unwrap_or_else(|_| panic!("Time went backwards"));
    since_the_epoch.as_secs()
}
