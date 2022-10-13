use chrono::{offset::Utc, DateTime};

pub fn calculate_latency(start: DateTime<Utc>, end: DateTime<Utc>) -> i64 {
    let start_ts = start.timestamp();
    let start_ts_ss = start.timestamp_subsec_millis() as i64;
    let end_ts = end.timestamp();
    let end_ts_ss = end.timestamp_subsec_millis() as i64;

    ((end_ts - start_ts) * 1000) + (end_ts_ss - start_ts_ss)
}
