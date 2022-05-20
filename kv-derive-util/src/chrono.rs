#[cfg(feature = "chrono")]
use chrono::{DateTime, TimeZone, Utc};
use kv_derive::{FromRepr, IntoRepr};

#[cfg(feature = "chrono")]
#[derive(FromRepr, IntoRepr, Debug, Default)]
pub struct DateTimeAsTimestamp(pub i64);

#[cfg(feature = "chrono")]
impl<Tz: TimeZone> From<DateTime<Tz>> for DateTimeAsTimestamp {
    fn from(datetime: DateTime<Tz>) -> Self {
        Self(datetime.timestamp())
    }
}

#[cfg(feature = "chrono")]
impl From<DateTimeAsTimestamp> for DateTime<Utc> {
    fn from(this: DateTimeAsTimestamp) -> Self {
        Utc.timestamp(this.0, 0)
    }
}
