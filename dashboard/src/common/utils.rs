use std::time::{Duration, UNIX_EPOCH};

use chrono::{DateTime, FixedOffset, Utc};

pub fn format_time(timestamp: i64) -> String {
    let d = UNIX_EPOCH + Duration::from_secs(timestamp as u64);
    let mut datetime = DateTime::<Utc>::from(d);
    let datetime_offset = datetime.with_timezone(&FixedOffset::east_opt(3 * 3600).unwrap());
    datetime_offset.format("%H:%M:%S").to_string()
}
