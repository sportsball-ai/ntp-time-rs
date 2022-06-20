use chrono::{DateTime, NaiveDateTime, Utc};

// 1970 - 1900 in seconds
const NTP_JAN_1970: u32 = 2_208_988_800;
// 2^32 = 4294967296 as a double
const NTP_FRAC: f64 = (1u64 << 32) as f64;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct NTPTimestamp {
    seconds: u32,
    fraction: u32,
}

impl From<DateTime<Utc>> for NTPTimestamp {
    fn from(datetime: DateTime<Utc>) -> Self {
        let seconds = datetime.timestamp() as u32 + NTP_JAN_1970;
        let fraction = (datetime.timestamp_subsec_nanos() as f64 * NTP_FRAC / 1e9) as u32;
        Self { seconds, fraction }
    }
}

impl From<NTPTimestamp> for u64 {
    fn from(ntp: NTPTimestamp) -> Self {
        (ntp.seconds as u64) << 32 | ntp.fraction as u64
    }
}

pub fn convert_raw_ntp_timestamp_to_utc(ntp_time: u64) -> DateTime<Utc> {
    let seconds = (ntp_time >> 32) as i64 - NTP_JAN_1970 as i64;
    let sub_nano_seconds = ((ntp_time as u32) as f64 * 1e9 / NTP_FRAC) as u32;
    let dt = NaiveDateTime::from_timestamp(seconds, sub_nano_seconds);
    DateTime::from_utc(dt, Utc)
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{NaiveDate, NaiveTime};
    #[test]
    fn test_ntp_timestamps() {
        let dt = NaiveDateTime::new(NaiveDate::from_ymd(2022, 6, 20), NaiveTime::from_hms_micro(8, 12, 30, 9000));
        let now = DateTime::from_utc(dt, Utc);
        let ntp_time: NTPTimestamp = now.into();
        assert_eq!(
            ntp_time,
            NTPTimestamp {
                seconds: 3864701550,
                fraction: 38654705
            }
        );

        let delta = now - convert_raw_ntp_timestamp_to_utc(ntp_time.into());
        assert!(delta.num_nanoseconds().unwrap().abs() <= 1);
    }
}
