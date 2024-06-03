use chrono::prelude::*;
use chrono::TimeDelta;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Serialize, Deserialize)]
pub struct Day {
    pub date: NaiveDate,
    pub start: Option<NaiveDateTime>,
    pub stop: Option<NaiveDateTime>,
    #[serde(deserialize_with = "deserialize_timedelta")]
    #[serde(serialize_with = "serialize_timedelta")]
    pub lunch: Option<TimeDelta>,
}

impl Day {
    /// Combines two Days into a third.
    ///
    /// The "other" variable overwrites the first.
    pub fn combine(&self, other: Day) -> Day {
        assert!(self.date == other.date);
        Day {
            date: self.date,
            start: other.start.or(self.start),
            stop: other.stop.or(self.stop),
            lunch: other.lunch.or(self.lunch),
        }
    }
}

fn serialize_timedelta<S>(timedelta: &Option<TimeDelta>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    // Check if the option is Some
    if let Some(timedelta) = timedelta {
        // Serialize the total number of seconds in the timedelta
        serializer.serialize_i64(timedelta.num_seconds())
    } else {
        // If the option is None, serialize it as None
        serializer.serialize_none()
    }
}
fn deserialize_timedelta<'de, D>(deserializer: D) -> Result<Option<TimeDelta>, D::Error>
where
    D: Deserializer<'de>,
{
    let int_or_none = Option::deserialize(deserializer)?;
    let timedelta_or_none = int_or_none.map(|seconds| TimeDelta::try_seconds(seconds).unwrap());
    Ok(timedelta_or_none)
}
