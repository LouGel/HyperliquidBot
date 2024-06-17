use crate::handlers::constants_callbacks::*;
use crate::DEAD_CALLBACK;
use teloxide::types::{
    InlineKeyboardButton, InlineKeyboardButtonKind::CallbackData, InlineKeyboardMarkup,
};
use teloxide_core::types::Message;

use crate::InlineKeyBoardHandler;

impl InlineKeyBoardHandler for InlineKeyboardMarkup {
    fn create_from_msg(message: &Message) -> InlineKeyboardMarkup {
        message.reply_markup().unwrap().clone()
    }

    fn update_green_checks_on_buttons(&mut self, action: String, value_to_find: String) -> bool {
        debug!("Action : {}", action);
        for row in &mut self.inline_keyboard {
            for button in row {
                if let CallbackData(callback_data) = &button.kind {
                    if !callback_data.contains(&action) {
                        continue;
                    }
                }
                let mut button_text = button.text.clone();

                if button_text.contains("✅ ") {
                    if button.text.contains(&value_to_find) {
                        return false;
                    }
                    if let CallbackData(callback_data) = &button.kind {
                        let new_callback_data = callback_data.replace(DEAD_CALLBACK, "");
                        debug!("New callback data for older value : {}", new_callback_data);
                        if callback_data.contains(CUSTOM) {
                            button_text = "Custom".to_owned();
                        } else {
                            button_text = button_text.replace("✅ ", "");
                        }
                        *button = InlineKeyboardButton::callback(button_text, new_callback_data);
                    }
                } else if button.text.contains(&value_to_find) {
                    if let CallbackData(callback_data) = &button.kind {
                        let button_text_buffer = format!("✅ {}", button.text);
                        let mut new_callback_data = callback_data.to_string();
                        new_callback_data.push('!');
                        debug!("New callback data  for value pusher: {}", new_callback_data);

                        *button =
                            InlineKeyboardButton::callback(button_text_buffer, new_callback_data);
                    }
                }
            }
        }

        true
    }
    fn get_value_from_callback_fct(&self, callback_fct: &str) -> Option<String> {
        let unique_setter = callback_fct.contains("Set");
        for row in self.inline_keyboard.iter() {
            for button in row.iter() {
                if let CallbackData(data) = &button.kind {
                    if data.contains(&callback_fct) {
                        if !unique_setter {
                            return Some(button.text.replace('✅', ""));
                        } else {
                            return Some(button.text.clone());
                        }
                    }
                }
            }
        }
        None
    }
    fn get_result_from_callback_fct(&self, callback_fct: &str) -> anyhow::Result<String> {
        self.get_value_from_callback_fct(callback_fct)
            .ok_or(anyhow::anyhow!("Couldn't get callback {callback_fct}"))
    }
    fn get_value_from_checked_callback_fct(&self, callback_fct: &str) -> Option<String> {
        for row in self.inline_keyboard.iter() {
            for button in row.iter() {
                if let CallbackData(data) = &button.kind {
                    if data.contains(&callback_fct) && button.text.contains('✅') {
                        debug!("Button text : {}", button.text);
                        return Some(button.text.replace("✅ ", ""));
                    }
                }
            }
        }
        None
    }
    fn get_which_order_type(&self) -> anyhow::Result<(bool, bool)> {
        let first_button = self
            .inline_keyboard
            .get(0)
            .ok_or(anyhow::anyhow!("No first keyboard value"))?
            .get(0)
            .ok_or(anyhow::anyhow!("No first keyboard value"))?;
        if let CallbackData(callback_data) = first_button.kind.clone() {
            debug!("Callback data :{}", callback_data);
            return Ok((callback_data.contains(BUY), callback_data.contains(LIMIT)));
        } else {
            Err(anyhow::anyhow!("No callback data in first message"))
        }
    }
    fn get_result_from_checked_callback_fct(&self, callback_fct: &str) -> anyhow::Result<String> {
        self.get_value_from_checked_callback_fct(callback_fct)
            .ok_or(anyhow::anyhow!("Handled: Couldn't get {callback_fct}"))
    }

    fn change_text_where_callback_contains(&mut self, callback_fct: &str, new_title: &str) {
        debug!("In change text where callback contain {callback_fct}, {new_title}");
        for row in self.inline_keyboard.iter_mut() {
            for button in row.iter_mut() {
                if let CallbackData(data) = &button.kind {
                    if data.contains(callback_fct) {
                        button.text = new_title.to_string();
                    }
                }
            }
        }
    }
}
