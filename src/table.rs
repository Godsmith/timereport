use crate::day::Day;
#[cfg(feature = "mock-open")]
use crate::mockopen::open;
use build_html::Html;
use chrono::prelude::*;
use chrono::TimeDelta;
#[cfg(not(feature = "mock-open"))]
use open;
use std::collections::HashMap;
use std::fs;
use std::io::Error;
use std::thread::sleep;
use std::time;
use tabled::builder::Builder;
use tabled::grid::records::vec_records::Cell;
use tabled::grid::records::Records;
use tempfile::tempdir;

pub fn create_week_table(
    date: NaiveDate,
    day_from_date: HashMap<NaiveDate, Day>,
    show_weekend: bool,
) -> String {
    create_table(date, day_from_date, show_weekend).to_string()
}

/// Currently does not delete the file automatically.
pub fn create_html_week_table(
    date: NaiveDate,
    day_from_date: HashMap<NaiveDate, Day>,
    show_weekend: bool,
) -> Result<(), Error> {
    let html = to_html_table(create_table(date, day_from_date, show_weekend)).to_html_string();
    let tmp_dir = tempdir()?;
    let path = tmp_dir.path().join("tmp.html");
    fs::write(&path, html)?;
    open::that(path)?;
    // Sleep here so that the browser has time to load the file before it
    // is deleted. Kind of hacky.
    sleep(time::Duration::from_millis(200));

    Ok(())
}

fn to_html_table(table: tabled::Table) -> build_html::Table {
    let mut rows: Vec<Vec<&str>> = vec![];
    for tabled_row in table.get_records().iter_rows() {
        let mut html_row: Vec<&str> = vec![];
        for cell in tabled_row.iter() {
            html_row.push(cell.text())
        }
        rows.push(html_row)
    }
    build_html::Table::from(rows)
}

fn create_table(
    date_to_display: NaiveDate,
    day_from_date: HashMap<NaiveDate, Day>,
    show_weekend: bool,
) -> tabled::Table {
    let mut builder = Builder::default();
    let week_days = days_in_week_of(date_to_display, show_weekend);
    builder.push_record(date_row(&week_days, "%Y-%m-%d")); // date
    builder.push_record(date_row(&week_days, "%A")); // weekday

    let mut start_row = vec!["start".to_string()];
    start_row.extend(starts(&week_days, &day_from_date));
    builder.push_record(start_row);

    let mut stop_row = vec!["stop".to_string()];
    stop_row.extend(stops(&week_days, &day_from_date));
    builder.push_record(stop_row);

    let mut lunch_row = vec!["lunch".to_string()];
    lunch_row.extend(lunches(&week_days, &day_from_date));
    builder.push_record(lunch_row);

    builder.build()
}

fn date_row(week_days: &Vec<NaiveDate>, format: &str) -> Vec<String> {
    let mut strings: Vec<String> = week_days
        .iter()
        .map(|date| date.format(format).to_string())
        .collect();
    strings.insert(0, "".to_string());
    strings
}

fn starts(week_days: &Vec<NaiveDate>, days: &HashMap<NaiveDate, Day>) -> Vec<String> {
    week_days
        .iter()
        .map(|date| match days.get(date) {
            None => "".to_string(),
            Some(day) => match day.start {
                None => "".to_string(),
                Some(dt) => dt.format("%H:%M").to_string(),
            },
        })
        .collect()
}

fn stops(week_days: &Vec<NaiveDate>, days: &HashMap<NaiveDate, Day>) -> Vec<String> {
    week_days
        .iter()
        .map(|date| match days.get(date) {
            None => "".to_string(),
            Some(day) => match day.stop {
                None => "".to_string(),
                Some(dt) => dt.format("%H:%M").to_string(),
            },
        })
        .collect()
}

fn lunches(week_days: &Vec<NaiveDate>, days: &HashMap<NaiveDate, Day>) -> Vec<String> {
    week_days
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
        .collect()
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
