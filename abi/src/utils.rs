use chrono::{DateTime, NaiveDateTime, Utc};
use prost_types::Timestamp;

pub fn convert_to_utc_time(ts: Timestamp) -> DateTime<Utc> {
    DateTime::<Utc>::from_utc(
        NaiveDateTime::from_timestamp_opt(ts.seconds, ts.nanos as _).unwrap(),
        Utc,
    )
}

pub fn convert_to_timestamp(ds: DateTime<Utc>) -> Timestamp {
    Timestamp {
        seconds: ds.timestamp(),
        nanos: ds.timestamp_subsec_nanos() as _,
    }
}
