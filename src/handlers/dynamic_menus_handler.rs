use crate::traits::InlineKeyBoardHandler;
use crate::{bot::modify_buttons, send_unexpected_error};
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
            modify_buttons(bot, user.id, msg.id, keyboard);
        } else {
            send_unexpected_error(bot, &user, format!("wrong callback {:?}", menu))
        }
    }
    Ok(())
}
