use crate::timedelta::{
    deserialize_hashmap_timedelta, deserialize_option_timedelta, serialize_hashmap_timedelta,
    serialize_option_timedelta,
};
use chrono::prelude::*;
use chrono::TimeDelta;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
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

#[cfg(test)]
mod test {
    use std::collections::HashMap;

    use chrono::{NaiveDate, NaiveDateTime, TimeDelta};
    use rstest::rstest;

    use crate::day::Day;

    #[rstest]
    fn debug() {
        // Create a sample Day instance
        let day = Day {
            date: NaiveDate::from_ymd_opt(2025, 2, 17).expect(""),
            start: Some(
                NaiveDateTime::parse_from_str("2025-02-17 08:00:00", "%Y-%m-%d %H:%M:%S")
                    .expect(""),
            ),
            stop: Some(
                NaiveDateTime::parse_from_str("2025-02-17 17:00:00", "%Y-%m-%d %H:%M:%S")
                    .expect(""),
            ),
            lunch: Some(TimeDelta::zero()),
            projects: HashMap::new(),
        };

        // Format the Day instance using Debug
        let debug_output = format!("{:?}", day);

        // Define the expected output string
        let expected = r#"Day { date: 2025-02-17, start: Some(2025-02-17T08:00:00), stop: Some(2025-02-17T17:00:00), lunch: Some(TimeDelta { secs: 0, nanos: 0 }), projects: {} }"#;

        // Assert that the Debug output matches the expected format
        assert_eq!(debug_output, expected);
    }
}
