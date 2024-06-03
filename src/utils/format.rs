pub fn format_float<T: std::fmt::Display>(input: T, max_decimals: usize) -> String {
    let input_str = input.to_string();
    if let Some(dot_index) = input_str.find('.') {
        let (int_part, dec_part) = input_str.split_at(dot_index);
        let dec_part_truncated = &dec_part[..(max_decimals + 1).min(dec_part.len())]; // +1 to include the dot itself
        format!("{}{}", int_part, dec_part_truncated)
            .trim_end_matches('0')
            .trim_end_matches('.')
            .to_string()
    } else {
        input_str
    }
}
