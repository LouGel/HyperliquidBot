use crate::globals::*;
use crate::hyperliquid_api::*;
use crate::menus::sell_menu::*;
// use crate::parse_slippage;
use crate::traits::{PKeyHandler, TxLink};
use crate::AddressForBot;
use anyhow::Result;
use ethers::types::TxHash;
use ethers::types::U256;

use ethers::providers::Middleware;
use std::sync::Arc;
use teloxide::{
    prelude::*,
    types::{InlineKeyboardMarkup, User},
};
use url::Url;
pub async fn sell_from_menu(_bot: &Bot, user: &User, menu: InlineKeyboardMarkup) -> Result<Url> {
    let SellMenuObject {
        chain,
        slippage,
        wallet_index,
        amount,
        token,
    } = get_values_from_sell_markup(menu)?;

    todo!("Sell fron menu")
    // let tx_hash: TxHash = execute_swap(signer, builded_params, Some(address_formatted)).await?;
    // debug!("Tx swap {}", tx_hash);
    // tx_hash.to_tx_link(network)
}
