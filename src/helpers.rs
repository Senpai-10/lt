// https://docs.rs/chrono/0.4.0/chrono/format/strftime/index.html

use chrono::prelude::DateTime;
use colored::ColoredString;
use chrono::Local;
use nanoid::nanoid;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

pub fn generate_id(length: usize) -> String {
    let charset: Vec<char> = [
        // '1', '2', '3', '4', '5', '6', '7', '8', '9', '0', 'a', 'b', 'c', 'd', 'e', 'f',
        '1', '2', '3', '4', '5', '6', '7', '8', '9', '0'
    ].to_vec();

    let id = nanoid!(length, &charset);

    return id;
}

pub fn get_unix_timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

pub fn convert_unix_timestamp(timestamp: u64, date_format: &String) -> String {
    let d = UNIX_EPOCH + Duration::from_secs(timestamp);
    let datetime = DateTime::<Local>::from(d);

    datetime.format(date_format).to_string()
}

pub fn calculate_percentage(part: i32, whole: i32) -> i32 {
    if part == 0 || whole == 0 {
        return 0;
    }

    100 * part / whole
}

/// Cut string off and add a string at the end e.g '...'
/// Suffix is a str to put after `max_length`
pub fn truncate_with_suffix(s: &mut String, max_length: usize, suffix: ColoredString) {
    if max_length == 0 { return }

    if s.len() > max_length {
        s.truncate(max_length);
        s.push_str(suffix.to_string().as_str());
    }
}

