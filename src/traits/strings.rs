use anyhow::Result;

pub trait OmnixString {
    fn clean_and_parse_to_float(self) -> Result<f64>;
    fn clean_and_parse_to_usize(self) -> Result<usize>;
    fn extract_hex_error_after_pattern(self, pattern: &str) -> Result<String>;
    fn from_str_to_sec(self) -> Result<u64>;
    fn parse_and_scale_percentage(self) -> Result<u16>;
}
