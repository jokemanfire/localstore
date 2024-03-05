use chrono::{DateTime, NaiveDateTime, NaiveTime, TimeZone, Timelike, Utc};
use std::time::SystemTime;
use ring::digest::{self, Digest};

pub fn string_to_systemtime(date_time_str: &str) -> Result<SystemTime, chrono::ParseError> {
    let naive_date_time: NaiveDateTime =
        NaiveDateTime::parse_from_str(date_time_str, "%Y-%m-%d %H:%M:%S")?;

    let utc_date_time: DateTime<Utc> = Utc.from_utc_datetime(&naive_date_time);

    let system_time = utc_date_time.into();

    Ok(system_time)
}


pub fn compute_sha256(data: &[u8]) -> String {
    let binding = digest::digest(&digest::SHA256, data);
    let actual = binding.as_ref();
    vec_to_hex_string(actual)
}

fn vec_to_hex_string(vec: &[u8]) -> String {
    hex::encode(vec)
}
