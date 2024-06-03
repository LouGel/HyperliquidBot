use crate::handlers::constants_callbacks::*;
use crate::DEAD_CALLBACK;
use anyhow::anyhow;
use teloxide::types::{
    InlineKeyboardButton, InlineKeyboardButtonKind::CallbackData, InlineKeyboardMarkup,
};
use teloxide_core::types::Message;

use crate::{expert_keyboard_markup, InlineKeyBoardHandler};

impl InlineKeyBoardHandler for InlineKeyboardMarkup {
    fn create_from_msg(message: &Message) -> InlineKeyboardMarkup {
        message.reply_markup().unwrap().clone()
    }
    fn toggle_pro_mode(&mut self, menu_from: &str) -> anyhow::Result<()> {
        let mut activated: Option<bool> = None;
        for row in self.inline_keyboard.iter_mut() {
            for button in row.iter_mut() {
                {
                    if button.text.contains("ExpertMode ❌") {
                        button.text = "ExpertMode ✅".to_string();
                        activated = Some(true);
                        break;
                    } else if button.text.contains("ExpertMode ✅") {
                        button.text = "ExpertMode ❌".to_string();
                        activated = Some(false);
                        break;
                    }
                }
            }
        }
        match activated {
            Some(true) => self.append_rows_at_index(expert_keyboard_markup(menu_from), 7),
            Some(false) => self.remove_rows_at_index(7, expert_keyboard_markup(menu_from).len()),
            None => return Err(anyhow!("Couldn't activate pro mode")),
        }
        Ok(())
    }
    fn append_rows_at_index(&mut self, rows: Vec<Vec<InlineKeyboardButton>>, index: usize) {
        if index > self.inline_keyboard.len() {
            panic!("Index out of bounds");
        }
        self.inline_keyboard.splice(index..index, rows.into_iter());
    }
    fn remove_rows_at_index(&mut self, start_index: usize, length: usize) {
        if start_index >= self.inline_keyboard.len()
            || start_index + length > self.inline_keyboard.len()
        {
            panic!(
                "Index out of bounds or length too large. inline len : {}\n total index {}",
                self.inline_keyboard.len(),
                start_index + length
            );
        }
        let _removed_elements = self
            .inline_keyboard
            .drain(start_index..start_index + length);
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
        // // First, determine the indices to update without mutating anything.
        // let row_to_update = match self.get_row_to_update(&callback_fct) {
        //     Ok(Some(index)) => Some(index),
        //     _ => None,
        // };

        // let additional_row_index =
        //     row_to_update.map(|index| (index as i8 + other_index_rel) as usize);

        // // Now, check if the indices are valid and perform updates separately.
        // if let Some(index) = row_to_update {
        //     Self::clean_and_replace(&mut self.inline_keyboard[index], None, &callback_fct);
        // }

        // if let Some(other_index) = additional_row_index {
        //     // Ensure we are not trying to modify the same row twice
        //     if other_index != row_to_update.unwrap_or(usize::MAX) {
        //         if let Some(additional_row) = self.inline_keyboard.get_mut(other_index) {
        //             Self::clean_and_replace(additional_row, None, &callback_fct);
        //         }
        //     }
        // }

        // row_to_update.is_some()
        true
    }
    fn update_custom_fct(&mut self, custom_callback_group: String, text_to_custom: &str) -> bool {
        for row in self.inline_keyboard.iter_mut() {
            for button in row.iter_mut() {
                if let CallbackData(data) = &button.kind {
                    if data.contains(&custom_callback_group) {
                        if button.text.contains('✅') && !data.contains(CUSTOM) {
                            button.text = button.text.replace('✅', "");
                        } else if data.contains(CUSTOM) {
                            button.text = "✅ ".to_owned() + text_to_custom;
                        }
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
    fn get_result_from_checked_callback_fct(&self, callback_fct: &str) -> anyhow::Result<String> {
        self.get_value_from_checked_callback_fct(callback_fct)
            .ok_or(anyhow::anyhow!("Handled: Couldn't get {callback_fct}"))
    }
    // fn clean_and_replace(
    //     row: &mut Vec<InlineKeyboardButton>,
    //     additional_row_opt: Option<&mut Vec<InlineKeyboardButton>>,
    //     callback_fct: &str,
    // ) {
    //     let clean_and_mark = |button: &mut InlineKeyboardButton| {
    //         if let CallbackData(data) = &button.kind {
    //             if button.text.contains('✅') {
    //                 if data.contains(CUSTOM) {
    //                     button.text = "Custom".to_string();
    //                 } else {
    //                     button.text = button.text.replace("✅ ", "");
    //                 }
    //             }
    //             if data == callback_fct {
    //                 button.text = "✅ ".to_owned() + &button.text; // Note the space for better readability
    //             }
    //         }
    //     };

    //     row.iter_mut().for_each(|button| clean_and_mark(button));

    //     if let Some(additional_row) = additional_row_opt {
    //         additional_row
    //             .iter_mut()
    //             .for_each(|button| clean_and_mark(button));
    //     }
    // }

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

    //Return none if the button was previously called
    // fn get_row_to_update(&self, callback_fct: &str) -> Result<Option<usize>, ()> {
    //     for (index, row) in self.inline_keyboard.iter().enumerate() {
    //         for button in row.iter() {
    //             if let CallbackData(data) = &button.kind {
    //                 if data == callback_fct {
    //                     if button.text.contains('✅') {
    //                         return Ok(None);
    //                     }
    //                     return Ok(Some(index));
    //                 }
    //             }
    //         }
    //     }
    //     Err(())
    // }
}
