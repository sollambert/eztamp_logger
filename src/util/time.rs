use std::time::{SystemTime, UNIX_EPOCH};

pub(crate) fn is_leap_year(year: i64) -> bool {
    (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0)
}

pub(crate) fn unix_to_utc(mut ts: u128) -> (i64, u32, u32, u32, u32, u32, u32) {
    // seconds per unit
    let milli_sec = 1000;
    let sec_min = 60 * milli_sec;
    let sec_hour = 60 * sec_min;
    let sec_day = 24 * sec_hour;

    // start from 1970
    let mut year: i64 = 1970;

    // compute year
    loop {
        let year_secs = if is_leap_year(year) { 366 } else { 365 } * sec_day;
        if ts >= year_secs {
            ts -= year_secs;
            year += 1;
        } else {
            break;
        }
    }

    // month lengths
    let mut month_lengths = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
    if is_leap_year(year) {
        month_lengths[1] = 29;
    }

    let mut month: u32 = 0;
    for (i, days) in month_lengths.iter().enumerate() {
        let month_secs = (*days as u128) * sec_day;
        if ts >= month_secs {
            ts -= month_secs;
            month += 1;
        } else {
            month = i as u32;
            break;
        }
    }

    let day = (ts / sec_day) + 1;
    ts %= sec_day;

    let hour = ts / sec_hour;
    ts %= sec_hour;

    let minute = ts / sec_min;
    ts %= sec_min;
    let second = ts / milli_sec;
    let millis = ts % milli_sec;

    (
        year,
        month + 1, // months are 1-based
        day as u32,
        hour as u32,
        minute as u32,
        second as u32,
        millis as u32
    )
}

pub(crate) fn utc_timestamp() -> String {
    let ts = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis();

    let (y, m, d, hh, mm, ss, ms) = unix_to_utc(ts);

    format!("{:04}-{:02}-{:02} {:02}:{:02}:{:02}:{:03} UTC", y, m, d, hh, mm, ss, ms)
}