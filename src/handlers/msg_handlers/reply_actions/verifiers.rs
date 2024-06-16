pub fn is_good_amount_format(_text: &str, msg: &mut String) -> bool {
    match msg.parse::<f64>() {
        Ok(num) => num > 0.0,
        Err(_) => false,
    }
}
pub fn is_good_token_name_or_number(_text: &str, msg: &mut String) -> bool {
    // if let None = extract_word_after_number(text, msg) {
    if let Ok(ret) = TOKEN_LIST.get_result(&msg.to_uppercase()) {
        *msg = ret.name.to_owned();
        return true;
    }
    false
}
// pub fn is_good_token_name(text: &str, msg: &mut String) -> bool {
//     msg.make_ascii_uppercase();
//     is_data_in_tokens_balance(text, msg)
// }

use regex::Regex;

use crate::TOKEN_LIST;
pub fn is_data_in_tokens_balance(input: &str, to_check: &str) -> bool {
    Regex::new(r"\d+\.\s*([^\d\s]+(?:\.[^\d\s]+)*)")
        .unwrap()
        .captures_iter(input)
        .any(|cap| {
            cap.get(1)
                .map_or(false, |data| data.as_str().contains(to_check))
        })
}
fn extract_word_after_number(text: &str, number: &mut String) -> Option<()> {
    number.parse::<u16>().ok()?;
    // Convert the number to a string and append a period to form the pattern to search for
    let search_pattern = format!("{}.", number);

    // Search for the pattern in the text
    let start_index = text.find(&search_pattern)?;
    // Find the end of the pattern to start searching for the next word
    let start_search_from = start_index + search_pattern.len();

    // Trim any leading whitespace from the remaining string and then split it by whitespace
    let remaining_text = &text[start_search_from..].trim_start();
    let next_word = remaining_text.split_whitespace().next();

    // Return the next word, if found, as a String
    *number = next_word.map(|word| word.to_string())?;
    Some(())
}

pub fn is_a_kyber_swap_order(text: &str, input_number: &mut String) -> Option<usize> {
    *input_number = input_number.trim().to_string();
    input_number.parse::<usize>().ok()?;

    let number_separator = "->";

    let line = find_line_with_input_number(text, input_number)?;
    let rank = find_number_between_w_and_separator(&line, &number_separator)?;
    rank.checked_sub(1)
}
pub fn find_line_with_input_number(text: &str, input_number: &str) -> Option<String> {
    // Construct a regex pattern to match the input_number within parentheses
    let pattern = format!(r"\(\s*{}\s*\)", regex::escape(input_number));
    let re = Regex::new(&pattern).unwrap();

    // Iterate through each line of the text
    text.lines()
        .find(|line| re.is_match(line))
        .map(|line| line.trim().to_string())
}
pub fn find_number_between_w_and_separator(text: &str, separator_str: &str) -> Option<usize> {
    // Escape the separator string to safely use it in the regex pattern
    let escaped_separator = regex::escape(separator_str);

    // Constructing the regex pattern. This pattern looks for 'W' followed by one or more digits (\d+),
    // and captures those digits until it hits the specified separator string.
    let pattern = format!(r"W(\d+){}", escaped_separator);
    let re = Regex::new(&pattern).unwrap();

    // Search the text for a match to the pattern.
    re.captures(text).and_then(|caps| {
        // If a match is found, attempt to capture the group of digits immediately following 'W' and parse it as usize.
        caps.get(1)
            .map(|m| m.as_str().parse::<usize>().ok())
            .flatten()
    })
}

///////////////////////////////////////////////////////////////////
///////////////////////////////////////////////////////////////////
///////////////////////////////////////////////////////////////////
///////////////////////////////////////////////////////////////////
// pub fn is_good_duration_format(_text: &str, msg: &mut String) -> bool {
//     msg.retain(|c| c != ' ');

//     if let Some(last_char) = msg.chars().last() {
//         if DURATION_FORMAT.contains(last_char) {
//             let without_last_char = &msg[..msg.len() - last_char.len_utf8()];
//             if without_last_char.parse::<u32>().is_ok() {
//                 return true;
//             }
//         }
//     }
//     false
// }

// pub fn is_ethereum_public_key(_text: &str, s: &mut String) -> bool {
//     s.retain(|c| c != ' ');
//     // s = check_address_or_resolve_ens(input)
//     s.len() == 42
//         && s.starts_with("0x")
//         && s[2..]
//             .chars()
//             .all(|c| c.is_digit(16) || ('a' <= c && c <= 'f') || ('A' <= c && c <= 'F'))
// }

// pub fn is_good_percent_format(_text: &str, msg: &mut String) -> bool {
//     debug!("In goo percent format{}", &msg);
//     msg.retain(|c| c != ' ' && c != '%');
//     match msg.parse::<f64>() {
//         Ok(num) => {
//             *msg += "%";
//             num >= 0.0 && num <= 100.0
//         }
//         Err(_) => false,
//     }
// }
// pub fn is_good_neg_percent_format(_text: &str, msg: &mut String) -> bool {
//     debug!("In goo percent format{}", &msg);
//     msg.retain(|c| c != ' ' && c != '%' && c != '-');
//     match msg.parse::<f64>() {
//         Ok(num) => {
//             *msg = "-".to_string() + msg.as_ref() + "%";
//             num >= 0.0 && num <= 100.0
//         }
//         Err(_) => false,
//     }
// }
// pub fn is_good_pos_percent_format(_text: &str, msg: &mut String) -> bool {
//     debug!("In goo percent format{}", &msg);
//     msg.retain(|c| c != ' ' && c != '%' && c != '+');
//     match msg.parse::<f64>() {
//         Ok(num) => {
//             *msg = "+".to_string() + msg.as_ref() + "%";
//             num >= 0.0 && num <= 100.0
//         }
//         Err(_) => false,
//     }
// }
//const DURATION_FORMAT: &str = "hdy";
//
