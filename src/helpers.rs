// https://docs.rs/chrono/0.4.0/chrono/format/strftime/index.html

use chrono::prelude::DateTime;
use chrono::Local;
use nanoid::nanoid;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

pub fn generate_id(length: usize) -> String {
    let set: [char; 16] = [
        '1', '2', '3', '4', '5', '6', '7', '8', '9', '0', 'a', 'b', 'c', 'd', 'e', 'f',
    ];

    let id = nanoid!(length, &set);

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
