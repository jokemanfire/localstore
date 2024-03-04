use chrono::{DateTime, NaiveDateTime, NaiveTime, Timelike, Utc, TimeZone};
use std::time::SystemTime;

pub fn string_to_systemtime(date_time_str: &str) -> Result<SystemTime, chrono::ParseError> {
    // 假设字符串格式为 "YYYY-MM-DD HH:mm:ss"
    let naive_date_time: NaiveDateTime = NaiveDateTime::parse_from_str(date_time_str, "%Y-%m-%d %H:%M:%S")?;
    
    // 将NaiveDateTime转换为DateTime<Utc>
    let utc_date_time: DateTime<Utc> = Utc.from_utc_datetime(&naive_date_time);
    
    // 将DateTime<Utc>转换为SystemTime
    let system_time = utc_date_time.into();
    
    Ok(system_time)
}