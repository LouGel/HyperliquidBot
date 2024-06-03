use crate::bot::modify_buttons;
use crate::traits::InlineKeyBoardHandler;
use teloxide::{
    prelude::*,
    types::{InlineKeyboardMarkup, User},
};

use super::constants_callbacks::TOGGLE_EXPERT;

pub async fn dynamic_menus_handler(
    bot: &Bot,
    user: User,
    menu: Vec<&str>,
    msg: Message,
    _callback_function: &str,
) -> anyhow::Result<()> {
    if let Some(&action) = menu.get(2) {
        let mut keyboard = InlineKeyboardMarkup::create_from_msg(&msg);
        if action.contains(TOGGLE_EXPERT) {
            keyboard.toggle_pro_mode(menu[1])?;
            modify_buttons(bot, user.id, msg.id, keyboard);
            return Ok(());
        }
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
