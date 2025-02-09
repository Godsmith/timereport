use chrono::prelude::*;
use chrono::TimeDelta;
use serde::de::MapAccess;
use serde::de::Visitor;
use serde::ser::SerializeMap;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::collections::HashMap;
use std::fmt;
use std::fmt::Debug;

#[derive(Serialize, Deserialize, Clone)]
pub struct Day {
    pub date: NaiveDate,
    pub start: Option<NaiveDateTime>,
    pub stop: Option<NaiveDateTime>,
    #[serde(deserialize_with = "deserialize_option_timedelta")]
    #[serde(serialize_with = "serialize_option_timedelta")]
    pub lunch: Option<TimeDelta>,
    #[serde(deserialize_with = "deserialize_hashmap_timedelta")]
    #[serde(serialize_with = "serialize_hashmap_timedelta")]
    pub projects: HashMap<String, TimeDelta>,
}

impl Debug for Day {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Day")
            .field("date", &self.date)
            .field("start", &self.start)
            .field("stop", &self.stop)
            .field("lunch", &self.lunch)
            .field("projects", &self.projects)
            .finish()
    }
}

impl Day {
    /// Combines two Days into a third.
    ///
    /// The "other" variable overwrites the first.
    pub fn combine(&self, other: &Day) -> Day {
        assert!(self.date == other.date);
        Day {
            date: self.date,
            start: other.start.or(self.start),
            stop: other.stop.or(self.stop),
            lunch: other.lunch.or(self.lunch),
            projects: self
                .projects
                .clone()
                .into_iter()
                .chain(other.projects.clone())
                .collect(),
        }
    }

    pub fn has_content(&self) -> bool {
        self.start.is_some()
            || self.stop.is_some()
            || self.lunch.is_some()
            || !self.projects.is_empty()
    }
}

fn serialize_option_timedelta<S>(
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

fn deserialize_option_timedelta<'de, D>(deserializer: D) -> Result<Option<TimeDelta>, D::Error>
where
    D: Deserializer<'de>,
{
    let int_or_none = Option::deserialize(deserializer)?;
    let timedelta_or_none = int_or_none.map(|seconds| TimeDelta::try_seconds(seconds).unwrap());
    Ok(timedelta_or_none)
}

fn serialize_hashmap_timedelta<S>(
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
fn deserialize_hashmap_timedelta<'de, D>(
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
