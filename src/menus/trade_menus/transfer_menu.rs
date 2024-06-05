// use crate::handlers::constants_callbacks::*;

// use crate::traits::PKeyHandler;
// use crate::traits::{InlineKeyBoardHandler, OmnixString, TxLink};
// use crate::{
//     display_balance, get_wallet_to_title_buttons_and_custom, globals::*, vec_3_p_keys_to_address,
// };
// use crate::{get_main_and_faq_banner, hyperliquid_api::*};
// use anyhow::Result;
// use ethers::types::Address;

// use teloxide::{
//     prelude::*,
//     types::{InlineKeyboardButton, InlineKeyboardMarkup, User},
// };
// use url::Url;

// pub async fn transfer_menu(user_id: UserId) -> anyhow::Result<(String, InlineKeyboardMarkup)> {
//     let chain_name = CHAIN_ON.get_result_for_user_id(user_id)?;

//     let native_symbol = "USD";
//     let pks = WALLETS_PKEY.get_result(user_id)?;

//     let array_of_balance = display_balance(vec_3_p_keys_to_address(&pks));

//     let text = format_transfer_message(
//         &chain_name,
//         Some(array_of_balance?),
//         &gas_price_in_gwei?.to_string(),
//     );
//     let keyboard = get_transfer_keyboard(&chain_name, &native_symbol);

//     Ok((text, keyboard))
// }
// use std::str::FromStr;
// #[derive(Default, Debug)]
// pub struct TransferMarkupData {
//     pub chain: String,
//     pub token: Option<Address>,
//     pub wallet_from_index: usize,
//     pub receiver: String,
//     pub amount: String,
// }
// fn get_values_from_transfer_markup(
//     keyboard: InlineKeyboardMarkup,
// ) -> anyhow::Result<TransferMarkupData> {
//     let mut data = TransferMarkupData::default();
//     let token_str = keyboard.get_result_from_checked_callback_fct(TOKEN_TO_SEND)?;

//     let wallet_from_str = keyboard.get_result_from_checked_callback_fct(WALLET)?;

//     data.chain = keyboard.get_result_from_callback_fct(CHANGE_NETWORK)?;
//     data.token = Address::from_str(&token_str.replace(" ", "")).ok();

//     data.wallet_from_index = wallet_from_str.clean_and_parse_to_usize()? - 1;

//     data.receiver = keyboard.get_result_from_checked_callback_fct(RECEIVER)?;

//     let amount = keyboard.get_result_from_checked_callback_fct(AMOUNT_PLAIN)?;
//     //TODO check if it's necessary the max
//     data.amount = match amount.contains(CUSTOM) {
//         false => MAX.to_owned(),
//         true => amount,
//     };
//     debug!("Data fetched :{:#?}", data);
//     Ok(data)
// }

// pub async fn transfer_from_menu(user: &User, menu: InlineKeyboardMarkup) -> Result<Url> {
//     let data = get_values_from_transfer_markup(menu)?;
//     let receiver = get_address_from_receiver(&user.id, data.receiver)?;
//     todo!()

//     // let pk = WALLETS_PKEY.get_pk_for_index(user.id, data.wallet_from_index)?;

//     // let network = &NETWORK_MAP.get_result(&data.chain.to_lowercase())?;
//     // let signer = network.create_http_signer(pk.to_owned())?;
//     // let amount_raw = parse_raw_amount(data.amount, network, data.token).await?;

//     // match data.token {
//     //     None => signer_transfer_to(receiver, amount_raw, signer.clone())
//     //         .await?
//     //         .to_tx_link(network),
//     //     Some(token_address) => {
//     //         signer_erc20_transfer_to(receiver, amount_raw, signer, token_address)
//     //             .await?
//     //             .to_tx_link(network)
//     //     }
//     // }
// }

// pub fn get_address_from_receiver(user_id: &UserId, receiver: String) -> Result<Address> {
//     match Address::from_str(&receiver) {
//         Ok(address) => Ok(address), // Successfully parsed as Address
//         Err(_) => match receiver.clean_and_parse_to_usize() {
//             Ok(num) => {
//                 let pk = WALLETS_PKEY.get_pk_for_index(*user_id, num - 1)?;
//                 Ok(pk.to_address())
//             }
//             Err(e) => return Err(e),
//         },
//     }
// }

// fn format_transfer_message(
//     chain_name: &str,
//     array_of_balance: Option<String>,
//     gas_price: &str,
// ) -> String {
//     let intro = format!(
//         "<b>🤖 Hyperliquid X</b>\n
//         <u>{}</u>\n",
//         chain_name
//     );

//     let outro = format!("<i>Gas price: <u>{}</u></i>", gas_price,);
//     match array_of_balance {
//         Some(text) => format!("{}{}{}", intro, text, outro),
//         None => format!("{}{}{}", intro, "Balances : -\n", outro),
//     }
// }

// use crate::{get_networks_title_and_button, get_wallet_from_title_and_buttons};
// pub fn get_transfer_keyboard(chain_name: &str, native_symbol: &str) -> InlineKeyboardMarkup {
//     let (network_title, network_button) = get_networks_title_and_button(chain_name, TRANSFER_MENU);
//     let (wallet_title, wallet_buttons) = get_wallet_from_title_and_buttons(TRANSFER_MENU);
//     let (to_title, to_buttons, to_custom) = get_wallet_to_title_buttons_and_custom(TRANSFER_MENU);
//     InlineKeyboardMarkup::new(vec![
//         get_main_and_faq_banner(),
//         vec![network_title],
//         vec![network_button],
//         vec![wallet_title],
//         wallet_buttons,
//         vec![to_title],
//         to_buttons,
//         vec![to_custom],
//         vec![InlineKeyboardButton::callback("TOKEN", DEAD_CALLBACK)],
//         vec![
//             InlineKeyboardButton::callback(
//                 &format!("✅ {native_symbol}"),
//                 &format!("{DYN_ACTION}_{TRANSFER_MENU}_{TOKEN_TO_SEND}"),
//             ),
//             InlineKeyboardButton::callback(
//                 "Custom",
//                 &format!("{REPLY_ACT}_{TRANSFER_MENU}_{TOKEN_TO_SEND}_{CUSTOM}"),
//             ),
//         ],
//         vec![InlineKeyboardButton::callback(
//             "TRANSFER AMOUNT",
//             DEAD_CALLBACK,
//         )],
//         vec![
//             InlineKeyboardButton::callback(
//                 "Custom",
//                 &format!("{REPLY_ACT}_{TRANSFER_MENU}_{AMOUNT_PLAIN}_{CUSTOM}"),
//             ),
//             InlineKeyboardButton::callback(
//                 "MAX",
//                 &format!("{DYN_ACTION}_{TRANSFER_MENU}_{AMOUNT_PLAIN}_{MAX}"),
//             ),
//             // InlineKeyboardButton::callback("Custom%",&format!("{REPLY_ACT}_{TRANSFER_MENU}_{AMOUNT_PERC}_{CUSTOM}"),
//         ],
//         vec![InlineKeyboardButton::callback(
//             "💳 Send Transfer",
//             &format!("{REPLY_ACT}_{TRANSFER_MENU}_{TRANSFER}"),
//         )],
//     ])
// }
