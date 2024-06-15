use teloxide::types::{InlineKeyboardButton, InlineKeyboardMarkup};
use teloxide_core::types::Message;

pub trait InlineKeyBoardHandler {
    fn update_green_checks_on_buttons(&mut self, action: String, value_to_find: String) -> bool;
    fn get_result_from_callback_fct(&self, callback_fct: &str) -> anyhow::Result<String>;
    fn get_result_from_checked_callback_fct(&self, callback_fct: &str) -> anyhow::Result<String>;
    fn get_value_from_checked_callback_fct(&self, callback_fct: &str) -> Option<String>;
    fn append_rows_at_index(&mut self, rows: Vec<Vec<InlineKeyboardButton>>, index: usize);
    fn remove_rows_at_index(&mut self, start_index: usize, length: usize);
    fn create_from_msg(message: &Message) -> InlineKeyboardMarkup;
    fn toggle_pro_mode(&mut self, menu_from: &str) -> anyhow::Result<()>;
    fn change_text_where_callback_contains(&mut self, callback_fct: &str, new_title: &str);
    fn update_custom_fct(&mut self, custom_callback_group: String, text_to_custom: &str) -> bool;
    fn get_value_from_callback_fct(&self, callaback_fct: &str) -> Option<String>;
    fn get_whic_order_type(&self) -> anyhow::Result<(/*is_buy */ bool, /*is_limit:*/ bool)>;
}
