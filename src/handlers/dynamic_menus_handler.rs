use crate::bot::modify_buttons;
use crate::traits::InlineKeyBoardHandler;
use teloxide::{
    prelude::*,
    types::{InlineKeyboardMarkup, User},
};

pub async fn dynamic_menus_handler(
    bot: &Bot,
    user: User,
    menu: Vec<&str>,
    msg: Message,
    _callback_function: &str,
) -> anyhow::Result<()> {
    if let Some(_) = menu.get(2) {
        let mut keyboard = InlineKeyboardMarkup::create_from_msg(&msg);
        if keyboard.update_green_checks_on_buttons(
            menu[menu.len() - 2].to_string(),
            menu.last().unwrap().to_string(),
        ) {
            debug!("Modified !",);
            modify_buttons(bot, user.id, msg.id, keyboard);
        } else {
            debug!("NOT Modified !",);
        }
    }
    Ok(())
}
