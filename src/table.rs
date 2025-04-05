use crate::day::Day;
use crate::naive_date::one_date_per_week;
use crate::traits::Parsable;
use chrono::prelude::*;
use chrono::TimeDelta;
use std::collections::HashMap;
use tabled::grid::records::vec_records::Cell as _;
use tabled::settings::object::Cell;
use tabled::settings::style::HorizontalLine;
use tabled::settings::themes::Colorization;
use tabled::settings::Color;
use tabled::{builder::Builder, settings::Style};

pub fn create_terminal_table(
    first_date: NaiveDate,
    last_date: NaiveDate,
    day_from_date: &HashMap<NaiveDate, Day>,
    previous_day_from_date: &HashMap<NaiveDate, Day>,
    show_weekend: bool,
    project_names: &Vec<String>,
    working_time_per_day: &TimeDelta,
) -> String {
    one_date_per_week(first_date, last_date)
        .iter()
        .map(|date| {
            create_terminal_table_string(
                date,
                day_from_date,
                previous_day_from_date,
                show_weekend,
                project_names,
                working_time_per_day,
            )
        })
        .collect::<Vec<_>>()
        .join("\n\n")
}

fn create_terminal_table_string(
    date_to_display: &NaiveDate,
    day_from_date: &HashMap<NaiveDate, Day>,
    previous_day_from_date: &HashMap<NaiveDate, Day>,
    show_weekend: bool,
    project_names: &Vec<String>,
    working_time_per_day: &TimeDelta,
) -> String {
    let mut current_table = create_table(
        *date_to_display,
        day_from_date,
        show_weekend,
        project_names,
        working_time_per_day,
    );
    let previous_table = create_table(
        *date_to_display,
        previous_day_from_date,
        show_weekend,
        project_names,
        working_time_per_day,
    );

    let records1 = current_table.get_records().to_vec();
    let records2 = previous_table.get_records().iter().collect::<Vec<_>>();

    for (row_idx, (row1, row2)) in records1.iter().zip(records2.iter()).enumerate() {
        for (col_idx, (cell1, cell2)) in row1.iter().zip(row2.iter()).enumerate() {
            if cell1.text() != cell2.text() {
                current_table.with(Colorization::exact(
                    [Color::BOLD],
                    Cell::new(row_idx, col_idx),
                ));
            }
        }
    }

    current_table
        .with(
            Style::rounded()
                .remove_horizontals()
                .horizontals([(2, HorizontalLine::inherit(Style::modern()))]),
        )
        .to_string()
}

pub fn create_table(
    date_to_display: NaiveDate,
    day_from_date: &HashMap<NaiveDate, Day>,
    show_weekend: bool,
    project_names: &Vec<String>,
    working_time_per_day: &TimeDelta,
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

    let mut default_project_row = vec!["1. Default project".to_string()];
    default_project_row.extend(default_project_timedeltas(&week_days, &day_from_date));
    builder.push_record(default_project_row);

    for (index, project_name) in project_names.iter().enumerate() {
        let mut row = vec![format!("{}. {}", index + 2, project_name.clone())];
        row.extend(project_timedeltas(
            &project_name,
            &week_days,
            &day_from_date,
        ));
        builder.push_record(row);
    }

    let mut flex_row = vec!["Flex".to_string()];
    flex_row.extend(flex(&week_days, &day_from_date, *working_time_per_day));
    builder.push_record(flex_row);

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
                Some(timedelta) => timedelta.to_hhmm(),
            },
        })
        .collect()
}

fn default_project_timedeltas(
    week_days: &Vec<NaiveDate>,
    days: &HashMap<NaiveDate, Day>,
) -> Vec<String> {
    week_days
        .iter()
        .map(|date| match days.get(date) {
            None => "".to_string(),
            Some(day) => match default_project_time(day) {
                Some(timedelta) => timedelta.to_hhmm(),
                None => "".to_string(),
            },
        })
        .collect()
}

fn default_project_time(day: &Day) -> Option<TimeDelta> {
    match (day.start, day.stop, day.lunch) {
        (Some(start), Some(stop), Some(lunch)) => {
            Some(stop - start - lunch - day.projects.values().sum())
        }
        _ => None,
    }
}

fn project_timedeltas(
    project_name: &String,
    week_days: &Vec<NaiveDate>,
    days: &HashMap<NaiveDate, Day>,
) -> Vec<String> {
    week_days
        .iter()
        .map(|date| match days.get(date) {
            None => "".to_string(),
            Some(day) => match day.projects.get(project_name) {
                None => "".to_string(),
                Some(timedelta) => timedelta.to_hhmm(),
            },
        })
        .collect()
}

fn flex(
    week_days: &Vec<NaiveDate>,
    days: &HashMap<NaiveDate, Day>,
    working_time_per_day: TimeDelta,
) -> Vec<String> {
    week_days
        .iter()
        .map(|date| match days.get(date) {
            None => "".to_string(),
            Some(day) => match (day.start, day.stop, day.lunch) {
                (Some(start), Some(stop), Some(lunch)) => {
                    (stop - start - lunch - working_time_per_day).to_hhmm()
                }
                _ => "".to_string(),
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
