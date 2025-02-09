use argparse::consume_after_target;
use argparse::consume_bool;
use argparse::consume_dates;
use argparse::consume_two_after_target;
use chrono::prelude::*;
use chrono::Duration;
use chrono::TimeDelta;
use std::collections::HashMap;
use std::path::Path;
mod traits;
use traits::Parsable;
mod argparse;
mod config;
mod day;
pub mod mockopen;
// Rust note: need to do pub table here since it is used in the binary crate main.rs
pub mod table;
mod timedelta;
use day::Day;

pub fn parse_date(text: &str) -> Result<NaiveDateTime, String> {
    let today = Local::now();
    let today_string = today.format("%Y-%m-%d").to_string();
    let time_string = if text.contains(":") {
        text.to_string()
    } else {
        format!("{text}:00")
    };
    let datetime_string = format!("{} {}", today_string, time_string);
    match NaiveDateTime::parse_from_str(&datetime_string, "%Y-%m-%d %H:%M") {
        Ok(dt) => Ok(dt),
        Err(e) => {
            Err(format!("Could not parse date string '{}'. Error: '{}'", text, e).to_string())
        }
    }
}

fn parse_projects(
    args: Vec<String>,
    project_names: &Vec<String>,
) -> Result<(HashMap<String, TimeDelta>, Vec<String>), String> {
    let (result, args) = consume_two_after_target("project", args);
    let (project, timedelta) = match result {
        Ok(option) => match option {
            Some((project, timedelta)) => (project, timedelta),
            None => return Ok((HashMap::new(), args)),
        },
        Err(message) => return Err(message),
    };
    let timedelta = match TimeDelta::from_str(&timedelta) {
        Ok(dt) => dt,
        Err(message) => return Err(message),
    };
    let mut map = HashMap::new();
    let project = if !project_names.contains(&project) {
        let project_index: usize = match project.parse() {
            Ok(project_index) => project_index,
            Err(_) => return Err(format!("Unknown project '{}'", project)),
        };
        if project_index == 0 {
            return Err("No project with index 0".to_string());
        }
        if project_index == 1 {
            return Err("Cannot report time on default project".to_string());
        }
        // -2 here since the first non-default project has index 2
        match project_names.get(project_index - 2) {
            Some(project_name) => project_name.to_string(),
            None => return Err(format!("No project with index {}", project_index)),
        }
    } else {
        project
    };
    map.insert(project, timedelta);
    Ok((map, args))
}

fn parse_days(
    args: Vec<String>,
    project_names: &Vec<String>,
) -> Result<(Vec<Day>, Vec<String>), String> {
    let (start, args_after_start) = consume_after_target("start", args);
    let start = match start {
        Ok(option) => match option {
            None => None,
            Some(text) => match parse_date(&text) {
                Ok(dt) => Some(dt),
                Err(e) => return Err(e),
            },
        },
        Err(error) => return Err(error),
    };

    let (stop, args_after_stop) = consume_after_target("stop", args_after_start);
    let stop = match stop {
        Ok(option) => match option {
            None => None,
            Some(text) => match parse_date(&text) {
                Ok(dt) => Some(dt),
                Err(e) => return Err(e),
            },
        },
        Err(error) => return Err(error),
    };

    let (lunch, args_after_lunch) = consume_after_target("lunch", args_after_stop);
    let lunch = match lunch {
        Ok(option) => match option {
            None => None,
            Some(text) => match TimeDelta::from_str(&text) {
                Ok(dt) => Some(dt),
                Err(e) => return Err(e),
            },
        },
        Err(error) => return Err(error),
    };

    let (dates, args_after_consume_date) = consume_dates(args_after_lunch);
    let (last, args_after_consume_last) = consume_bool("last", args_after_consume_date);
    let (projects, args_after_parse_projects) =
        match parse_projects(args_after_consume_last, project_names) {
            Ok((projects, args)) => (projects, args),
            Err(message) => return Err(message),
        };

    let dates = if dates.is_empty() {
        vec![Local::now().date_naive()]
    } else {
        dates
    };
    let days = dates
        .iter()
        .map(|date| {
            if last {
                *date - Duration::try_weeks(1).expect("hardcoded int")
            } else {
                *date
            }
        })
        .map(|date| Day {
            date,
            start,
            stop,
            lunch,
            projects: projects.clone(),
        })
        .collect();
    Ok((days, args_after_parse_projects))
}

fn show_week_table(
    day_from_date: HashMap<NaiveDate, Day>,
    date: NaiveDate,
    show_weekend: bool,
    project_names: Vec<String>,
) -> String {
    table::create_week_table(date, day_from_date, show_weekend, project_names)
}

fn show_week_table_html(
    day_from_date: HashMap<NaiveDate, Day>,
    date: NaiveDate,
    show_weekend: bool,
    project_names: Vec<String>,
) -> String {
    match table::create_html_week_table(date, day_from_date, show_weekend, project_names) {
        Ok(_) => "".to_string(),
        Err(error) => format!("Error: '{}'", error.to_string()),
    }
}

fn undo(path: &Path) -> String {
    let mut config = config::load_config(path);

    let date = match config.undo() {
        Ok(date) => date,
        Err(message) => return message,
    };
    config::save_config(&config, path);
    let show_weekend = matches!(date.weekday(), Weekday::Sat | Weekday::Sun);
    show_week_table(
        config.day_from_date(),
        date,
        show_weekend,
        config.project_names,
    )
}

fn redo(path: &Path) -> String {
    let mut config = config::load_config(path);
    let date = match config.redo() {
        Ok(date) => date,
        Err(message) => return message,
    };
    config::save_config(&config, path);
    let show_weekend = matches!(date.weekday(), Weekday::Sat | Weekday::Sun);
    show_week_table(
        config.day_from_date(),
        date,
        show_weekend,
        config.project_names,
    )
}

pub fn main(args: Vec<String>, path: &Path) -> String {
    let mut config = config::load_config(path); // TODO; rename to load
    let (has_undo, args) = consume_bool("undo", args);
    if has_undo {
        return undo(path);
    }
    let (has_redo, args) = consume_bool("redo", args);
    if has_redo {
        return redo(path);
    }
    let (project_name, args) = consume_after_target("add", args);
    match project_name {
        Ok(project_name) => match project_name {
            Some(project_name) => {
                config.add_project(project_name);
                config::save_config(&config, path); // TODO: make instance method
            }
            None => (),
        },
        Err(message) => return message,
    };

    let (show_weekend, args_after_show_weekend) = consume_bool("--weekend", args);
    let (show_html, args_after_show_html) = consume_bool("html", args_after_show_weekend);
    let (result_for_arg_after_show, args_after_consuming_show) =
        consume_after_target("show", args_after_show_html);

    let arg_after_show = match result_for_arg_after_show {
        Err(message) => return message,
        Ok(value_after_show) => value_after_show,
    };
    let today = Local::now().date_naive();
    match arg_after_show.as_deref() {
        None => {}
        Some(value) => match value {
            "week" => {
                if show_html {
                    return show_week_table_html(
                        config.day_from_date(),
                        today,
                        show_weekend,
                        config.project_names,
                    );
                } else {
                    return show_week_table(
                        config.day_from_date(),
                        today,
                        show_weekend,
                        config.project_names,
                    );
                }
            }
            _ => return format!("Unknown show command: {}", value),
        },
    };

    let result = parse_days(args_after_consuming_show, &config.project_names);
    let (days, args_after_parse_days) = match result {
        Ok((days, args)) => (days, args),
        Err(message) => return message,
    };
    let date_to_display = match days.as_slice() {
        [] => unreachable!("days cannot be empty"),
        [first, ..] => first.date,
    };
    for day in days {
        if day.has_content() {
            config.add_day(day);
        }
    }
    config::save_config(&config, path);
    if !args_after_parse_days.is_empty() {
        return format!(
            "Unknown or extra argument '{}'",
            args_after_parse_days.join(", ")
        );
    }
    show_week_table(
        config.day_from_date(),
        date_to_display,
        show_weekend,
        config.project_names,
    )
}
