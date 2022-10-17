use time::OffsetDateTime;

pub fn calculate_latency(start: OffsetDateTime, end: OffsetDateTime) -> i64 {
    let start_ts = start.unix_timestamp();
    let start_ts_ss = start.millisecond() as i64;
    let end_ts = end.unix_timestamp();
    let end_ts_ss = end.millisecond() as i64;

    ((end_ts - start_ts) * 1000) + (end_ts_ss - start_ts_ss)
}
