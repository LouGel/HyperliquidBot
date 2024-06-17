mod bot;
mod errors;
mod globals;
mod handlers;
mod hyperliquid_api;
mod init;
mod menus;
mod traits;
mod types;
mod utils;

use bot::*;
use dotenv::dotenv;
use errors::check_env;
use globals::*;
use handlers::{callback_handler, commands_handler, message_handler};
use init::init_omni_bot;
use menus::*;
use std::env;
use std::sync::Arc;
use teloxide::{prelude::*, types::ChatKind};
use traits::*;
use types::*;
use utils::*;
#[macro_use]
extern crate log;

#[tokio::main]
async fn main() {
    dotenv().ok();
    pretty_env_logger::init_timed();
    info!("Checking env variables");
    check_env();
    info!("Bot initiation");
    // Simple initiation of immutables
    {
        let _ = &*TOKEN_LIST;
    }
    // Check if okay
    info!("Bot instanciation");
    warn!("Bot instanciation");
    let bot = Bot::from_env();
    init_omni_bot().await;
    // Here we create a dependancy map : first branch take message (Comand ex:\start , and simple ones) , second is the callback handler
    let handler = dptree::entry()
        .branch(
            Update::filter_message()
                .filter(|msg: Message| matches!(msg.chat.kind, ChatKind::Private(_)))
                .branch(teloxide::filter_command::<Command, _>().endpoint(commands_handler))
                .branch(dptree::endpoint(message_handler)),
        )
        .branch(Update::filter_callback_query().endpoint(callback_handler));

    Dispatcher::builder(bot, handler)
        .default_handler(|upd| async move {
            warn!("Unhandled update: {:?}", upd);
        })
        .error_handler(LoggingErrorHandler::with_custom_text(
            "An error has occurred in the dispatcher",
        ))
        .build()
        .dispatch()
        .await;
}
