use std::collections::HashSet;
use std::time::{Duration, UNIX_EPOCH};

use chrono::{DateTime, FixedOffset, Utc};

pub fn format_time(timestamp: i64) -> String {
    let d = UNIX_EPOCH + Duration::from_secs(timestamp as u64);
    let mut datetime = DateTime::<Utc>::from(d);
    let datetime_offset = datetime.with_timezone(&FixedOffset::east_opt(3 * 3600).unwrap());
    datetime_offset.format("%H:%M:%S").to_string()
}

pub fn prepare_symbols_for_url(symbols: HashSet<String>) -> String {
    symbols.iter().fold("".to_owned(), |mut acc, x| {
        if !acc.is_empty() {
            acc.push_str(",")
        };
        acc.push_str(x);
        acc
    })
}

pub fn round_f64(before: f64) -> f64 {
    f64::trunc(before * 10000.0) / 10000.0
}

pub fn round_f64_str(str_f64: String) -> f64 {
    round_f64(str_f64.parse::<f64>().unwrap())
}