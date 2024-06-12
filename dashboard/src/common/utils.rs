use std::collections::HashSet;
use std::time::{Duration, UNIX_EPOCH};

use chrono::{DateTime, FixedOffset, Utc};

/// Format timestamp(i64) to user friendly String
pub fn format_time(timestamp: i64) -> String {
    let d = UNIX_EPOCH + Duration::from_secs(timestamp as u64);
    let datetime = DateTime::<Utc>::from(d);
    let datetime_offset = datetime.with_timezone(&FixedOffset::east_opt(3 * 3600).unwrap());
    datetime_offset.format("%H:%M:%S").to_string()
}
/// Concat symbol names for server requests
pub fn prepare_symbols_for_url(symbols: HashSet<String>) -> String {
    symbols.iter().fold("".to_owned(), |mut acc, x| {
        if !acc.is_empty() {
            acc.push(',')
        };
        acc.push_str(x);
        acc
    })
}
/// Round f64 value to 4 precision
pub fn round_f64(before: f64) -> f64 {
    f64::trunc(before * 10000.0) / 10000.0
}
/// Parse and Round f64 value to 4 precision
pub fn round_f64_str(str_f64: String) -> f64 {
    round_f64(str_f64.parse::<f64>().unwrap())
}
