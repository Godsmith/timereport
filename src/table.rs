use crate::day::Day;
use chrono::prelude::*;
use chrono::TimeDelta;
use std::collections::HashMap;
use tabled::builder::Builder;

pub fn create_week_table(
    date: NaiveDate,
    days: HashMap<NaiveDate, Day>,
    show_weekend: bool,
) -> String {
    let mut builder = Builder::default();
    let week_days = days_in_week_of(date, show_weekend);
    let mut top_row: Vec<String> = week_days
        .iter()
        .map(|date| date.format("%Y-%m-%d").to_string())
        .collect();
    top_row.insert(0, "".to_string());
    builder.push_record(top_row);

    let starts: Vec<String> = week_days
        .iter()
        .map(|date| match days.get(date) {
            None => "".to_string(),
            Some(day) => match day.start {
                None => "".to_string(),
                Some(dt) => dt.format("%H:%M").to_string(),
            },
        })
        .collect();

    let stops: Vec<String> = week_days
        .iter()
        .map(|date| match days.get(date) {
            None => "".to_string(),
            Some(day) => match day.stop {
                None => "".to_string(),
                Some(dt) => dt.format("%H:%M").to_string(),
            },
        })
        .collect();

    let lunches: Vec<String> = week_days
        .iter()
        .map(|date| match days.get(date) {
            None => "".to_string(),
            Some(day) => match day.lunch {
                None => "".to_string(),
                Some(timedelta) => format!(
                    "{}:{}",
                    timedelta.num_hours(),
                    timedelta.num_minutes() - timedelta.num_hours() * 60
                )
                .to_string(),
            },
        })
        .collect();

    let mut start_row = vec!["start".to_string()];
    start_row.extend(starts);
    builder.push_record(start_row);

    let mut stop_row = vec!["stop".to_string()];
    stop_row.extend(stops);
    builder.push_record(stop_row);

    let mut lunch_row = vec!["lunch".to_string()];
    lunch_row.extend(lunches);
    builder.push_record(lunch_row);

    builder.build().to_string()
}

fn days_in_week_of(date: NaiveDate, show_weekend: bool) -> Vec<NaiveDate> {
    let offset = date.weekday().num_days_from_monday();
    let timedelta_to_last_monday = TimeDelta::try_days(-i64::from(offset)).unwrap();
    let date_of_last_monday = date + timedelta_to_last_monday;
    let day_count = if show_weekend { 7 } else { 5 };
    let timedeltas = (0..=(day_count - 1))
        .map(|i| TimeDelta::try_days(i).unwrap())
        .map(|d| date_of_last_monday + d);
    timedeltas.collect::<Vec<_>>()
}
