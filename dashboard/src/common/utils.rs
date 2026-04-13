use std::collections::HashSet;
use std::time::{Duration, UNIX_EPOCH};

use chrono::{DateTime, FixedOffset, Utc};

/// Format timestamp(i64) to user friendly String
pub fn format_time(timestamp: i64) -> String {
    let secs = u64::try_from(timestamp).unwrap_or(0);
    let d = UNIX_EPOCH + Duration::from_secs(secs);
    let datetime = DateTime::<Utc>::from(d);
    let datetime_offset = datetime.with_timezone(
        &FixedOffset::east_opt(3 * 3600)
            .unwrap_or_else(|| FixedOffset::east_opt(0).expect("UTC+0 is always valid")),
    );
    datetime_offset.format("%H:%M:%S").to_string()
}
/// Concat symbol names for server requests
pub fn prepare_symbols_for_url(symbols: HashSet<String>) -> String {
    symbols.into_iter().collect::<Vec<_>>().join(",")
}
/// Round f64 value to 4 decimal places
pub fn round_f64(before: f64) -> f64 {
    (before * 10000.0).round() / 10000.0
}
/// Parse and round f64 value to 4 decimal places
pub fn round_f64_str(str_f64: &str) -> f64 {
    str_f64.parse::<f64>().map(round_f64).unwrap_or(0.0)
}
