use crate::TOKEN_LIST;

pub fn is_good_amount_format(_text: &str, msg: &mut String) -> bool {
    match msg.parse::<f64>() {
        Ok(num) => {
            *msg += " ✏️";
            num > 0.0
        }
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
