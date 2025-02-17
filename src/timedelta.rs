use crate::traits::Parsable;
use chrono::TimeDelta;
use regex::Regex;
use serde::de::MapAccess;
use serde::de::Visitor;
use serde::ser::SerializeMap;
use serde::{Deserialize, Deserializer, Serializer};
use std::collections::HashMap;
use std::fmt;

fn to_hours_and_minutes(text: &str) -> Result<(&str, &str), String> {
    // 8: 15
    let re = Regex::new(r"(\d+):?(\d\d)").unwrap();
    if let Some(captures) = re.captures(text) {
        let (_, groups): (&str, [&str; 2]) = captures.extract();
        return Ok((groups[0], groups[1]));
    }
    // 45m
    let re2 = Regex::new(r"(\d+)m").unwrap();
    if let Some(captures) = re2.captures(text) {
        let (_, groups): (&str, [&str; 1]) = captures.extract();
        return Ok((&"0", groups[0]));
    }
    // 8
    let re3 = Regex::new(r"(\d+)").unwrap();
    if let Some(captures) = re3.captures(text) {
        let (_, groups): (&str, [&str; 1]) = captures.extract();
        return Ok((groups[0], &"0"));
    }
    return Err(format!("Could not parse timedelta string '{}'.", text));
}

impl Parsable for TimeDelta {
    /// Convert a &str to a TimeDelta.
    ///
    /// # Examples
    ///
    /// - 08:15
    /// - 8:15
    /// - 45m
    /// - 8
    fn from_str(text: &str) -> Result<Self, String> {
        let (hours, minutes) = to_hours_and_minutes(text)?;
        // Only panics when the minutes or hours strings are not integers, which should
        // not happen because then they wouldn't have matched the regex
        let seconds = hours.parse::<i64>().unwrap() * 3600 + minutes.parse::<i64>().unwrap() * 60;
        // Only panics when out of bounds, so should be safe
        Ok(TimeDelta::new(seconds, 0).unwrap())
    }

    /// Convert the TimeDelta to a String on the format "HH:MM".
    fn to_hhmm(&self) -> String {
        let total_seconds = self.num_seconds();
        let is_negative = total_seconds < 0;
        let total_seconds = total_seconds.abs(); // Work with absolute value for calculations

        let hours = total_seconds / 3600;
        let minutes = (total_seconds % 3600) / 60;

        if is_negative {
            format!("-{:02}:{:02}", hours, minutes)
        } else {
            format!("{:02}:{:02}", hours, minutes)
        }
    }
}

pub fn serialize_timedelta<S>(timedelta: &TimeDelta, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_i64(timedelta.num_seconds())
}

pub fn deserialize_timedelta<'de, D>(deserializer: D) -> Result<TimeDelta, D::Error>
where
    D: Deserializer<'de>,
{
    let int_or_none = Option::deserialize(deserializer)?;
    let timedelta_or_none = TimeDelta::try_seconds(int_or_none.unwrap()).unwrap();
    Ok(timedelta_or_none)
}

pub fn serialize_option_timedelta<S>(
    timedelta: &Option<TimeDelta>,
    serializer: S,
) -> Result<S::Ok, S::Error>
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

pub fn deserialize_option_timedelta<'de, D>(deserializer: D) -> Result<Option<TimeDelta>, D::Error>
where
    D: Deserializer<'de>,
{
    let int_or_none = Option::deserialize(deserializer)?;
    let timedelta_or_none = int_or_none.map(|seconds| TimeDelta::try_seconds(seconds).unwrap());
    Ok(timedelta_or_none)
}

pub fn serialize_hashmap_timedelta<S>(
    timedeltas: &HashMap<String, TimeDelta>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let mut map = serializer.serialize_map(Some(timedeltas.len()))?;
    for (key, timedelta) in timedeltas {
        map.serialize_entry(key, &timedelta.num_seconds())?;
    }
    map.end()
}

// Deserialize HashMap<String, TimeDelta>
pub fn deserialize_hashmap_timedelta<'de, D>(
    deserializer: D,
) -> Result<HashMap<String, TimeDelta>, D::Error>
where
    D: Deserializer<'de>,
{
    struct HashMapTimeDeltaVisitor;

    impl<'de> Visitor<'de> for HashMapTimeDeltaVisitor {
        type Value = HashMap<String, TimeDelta>;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a map of strings to integers representing TimeDelta in seconds")
        }

        fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
        where
            A: MapAccess<'de>,
        {
            let mut timedeltas = HashMap::new();
            while let Some((key, seconds)) = map.next_entry::<String, i64>()? {
                let timedelta = TimeDelta::try_seconds(seconds).unwrap();
                timedeltas.insert(key, timedelta);
            }
            Ok(timedeltas)
        }
    }

    deserializer.deserialize_map(HashMapTimeDeltaVisitor)
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;

    use chrono::{NaiveDate, NaiveDateTime, TimeDelta};
    use rstest::rstest;

    use crate::{day::Day, timedelta::to_hours_and_minutes};

    #[rstest]
    fn parse_error() {
        let actual = to_hours_and_minutes("foo");

        assert_eq!(
            actual,
            Err("Could not parse timedelta string 'foo'.".to_string())
        );
    }
}
