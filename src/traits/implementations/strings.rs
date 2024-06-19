use crate::OmnixString;
use anyhow::Result;

impl OmnixString for &str {
    fn clean_and_parse_to_usize(self) -> Result<usize> {
        Ok(self
            .chars()
            .filter(|&c| c.is_digit(10))
            .collect::<String>()
            .parse::<usize>()?)
    }
    // fn from_str_to_sec(self) -> Result<u64> {
    //     let (value, unit) = self.split_at(self.len() - 1);

    //     let value = value.replace(",", ".");

    //     let value: f64 = match value.parse() {
    //         Ok(v) => v,
    //         Err(_) => return Err(anyhow!("Invalid number format")),
    //     };
    //     let duration_in_seconds = match unit {
    //         "y" => value * 365.25 * 24.0 * 3600.0,
    //         "d" => value * 24.0 * 3600.0,
    //         "h" => value * 3600.0,
    //         _ => return Err(anyhow!("Unsupported time unit")),
    //     };
    //     if duration_in_seconds.is_sign_positive() {
    //         Ok(duration_in_seconds as u64)
    //     } else {
    //         Err(anyhow!("Time duration must be positive"))
    //     }
    // }

    // fn parse_and_scale_percentage(self) -> Result<u16> {
    //     let value: f64 = self
    //         .replace('%', "")
    //         .parse()
    //         .map_err(|e| anyhow!("Error in  parse_and_scale_percentage :{:?}", e))?;
    //     let scaled_value = value.abs() * 100.0;
    //     if scaled_value >= 0.0 && scaled_value <= 10000.0 {
    //         Ok(scaled_value as u16)
    //     } else {
    //         Err(anyhow!("Value out of range"))
    //     }
    // }
    fn clean_and_parse_to_float(self) -> Result<f64> {
        Ok(self
            .chars()
            .filter(|&c| c.is_digit(10) || c == '.')
            .collect::<String>()
            .parse::<f64>()
            .map_err(|e| anyhow::anyhow!("Error in  clean and parse :{}", e.to_string()))?)
    }

    // fn extract_hex_error_after_pattern(self, pattern: &str) -> Result<String> {
    //     if let Some(start) = self.find(pattern) {
    //         let mut hex_data = self[start + pattern.len()..].to_string();
    //         if hex_data.len() % 2 != 0 {
    //             hex_data.insert(0, '0');
    //         }
    //         let bytes = hex::decode(&hex_data)?;
    //         let mut result = String::from_utf8_lossy(&bytes).into_owned();
    //         result.retain(|c| c != '\0');

    //         Ok(result)
    //     } else {
    //         Err(anyhow!("Pattern not found"))
    //     }
    // }
}
