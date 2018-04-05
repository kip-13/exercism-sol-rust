extern crate chrono;
use chrono::{DateTime, Utc, Duration};

pub fn after(start: DateTime<Utc>) -> DateTime<Utc> {
    let secs = 10i64.pow(9);

    start + Duration::seconds(secs)
}
