use crate::globals::*;
use crate::hyperliquid_api::*;
use crate::menus::buy_menu::*;
use crate::traits::{PKeyHandler, TxLink};
use crate::AddressForBot;
use anyhow::Result;
use teloxide::prelude::*;
use teloxide::types::*;
use url::Url;
pub async fn buy_from_menu(_bot: &Bot, user: &User, menu: InlineKeyboardMarkup) -> Result<Url> {
    let lol = get_values_from_buy_markup(menu)?;

    todo!("Buy_from_menu")

    // execute_swap(signer, builded_params, None)
    //     .await?
    //     .to_tx_link(network)
}
