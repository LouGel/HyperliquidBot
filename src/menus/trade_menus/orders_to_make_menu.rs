use crate::get_main_and_faq_banner;
use crate::get_wallet_from_title_and_buttons;
use crate::globals::*;
use crate::handlers::constants_callbacks::*;
use crate::traits::InlineKeyBoardHandler;
use crate::types::hyperliquid_client::{Balance, HyperLiquidNetwork};
use crate::vec_3_p_keys_to_address;
use crate::{modify_message_with_buttons, send_unexpected_error};
use std::time::{SystemTime, UNIX_EPOCH};
use teloxide::prelude::*;
use teloxide::types::*;

pub async fn make_orders_menu(
    user_id: UserId,
    token_name: &str,
    is_buy: bool,
    is_limit: bool,
) -> anyhow::Result<(String, InlineKeyboardMarkup)> {
    let p_ks = WALLETS_PKEY.get_result(user_id)?;
    let addresses = vec_3_p_keys_to_address(&p_ks);
    let client = HyperLiquidNetwork::get_client();
    let price = client.clone().fetch_price_for_token(token_name).await?;

    // let text = match (is_buy, is_limit) {
    //     (true, true) => todo!(),
    //     (true, false) => todo!(),
    //     (false, true) => todo!(),
    //     (false, false) => todo!(),
    // };
    let balances_raw = client.fetch_spot_balance_for_addresses(&addresses).await?;
    let mut balances_usdc = Vec::new();
    for balances in &balances_raw {
        let usdc_balance: Vec<&Balance> = balances.iter().filter(|x| x.coin == "USDC").collect();
        balances_usdc.push(usdc_balance.into_iter().next())
    }

    let inline_keyboard = get_order_keyboard("WAGMI", &price, is_buy, is_limit);
    let text = format_limit_buy_message(balances_usdc);
    Ok((text, inline_keyboard))
}

pub async fn make_orders_menu_from_keyboard(
    user: &User,
    keyboard: InlineKeyboardMarkup,
) -> anyhow::Result<(String, InlineKeyboardMarkup)> {
    let token_name: String = keyboard.get_result_from_callback_fct(TOKEN_NAME)?;
    let (is_buy, is_limit) = keyboard.get_whic_order_type()?;
    let (text, _) = make_orders_menu(user.id, &token_name, is_buy, is_limit).await?;
    Ok((text, keyboard))
}

pub async fn spawn_order_menu_from_keyboard(
    bot: &Bot,
    user: &User,
    msg_id: MessageId,
    keyboard: InlineKeyboardMarkup,
) {
    match make_orders_menu_from_keyboard(user, keyboard).await {
        Ok((text, keyboard)) => {
            modify_message_with_buttons(bot, user, msg_id, &text, &keyboard);
        }
        Err(e) => send_unexpected_error(bot, user, e.to_string() + "in spawn limit_buy"),
    }
}

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

pub fn get_order_keyboard(
    desired_token: &str,
    price_usd: &str,
    is_buy: bool,
    is_limit: bool,
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
    let mut keyboard = vec![
        vec![InlineKeyboardButton::callback(
            " ",
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
            "Amount ✏️",
            &format!("{REPLY_ACT}_{MAKE_ORDERS_MENU}_{AMOUNT_PLAIN}"),
        )],
        vec![InlineKeyboardButton::callback(
            &format!("TOKEN(⬇️ price {price_usd}$)"),
            DEAD_CALLBACK,
        )],
        vec![InlineKeyboardButton::callback(
            &format!("{desired_token}"),
            &format!("{REPLY_ACT}_{MAKE_ORDERS_MENU}_{TOKEN_NAME}"),
        )],
    ];
    if is_limit {
        keyboard.push(vec![InlineKeyboardButton::callback(
            &format!("Price"),
            DEAD_CALLBACK,
        )]);
        keyboard.push(vec![InlineKeyboardButton::callback(
            &format!("0"),
            &format!("{REPLY_ACT}_{MAKE_ORDERS_MENU}_{PRICE_WANTED}"),
        )])
    }
    keyboard.push(vec![InlineKeyboardButton::callback(
        "SEND TX",
        &format!("{REPLY_ACT}_{EXECUTE_ORDER}"),
    )]);
    InlineKeyboardMarkup::new(keyboard)
}

// true => InlineKeyboardButton::callback(
// "🚮 Delete order",
// &format!("{REPLY_ACT}_{BUY_LIMIT_MENU}_{CANCEL_ORDER}"),
// ),
