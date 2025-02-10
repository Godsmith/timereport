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
