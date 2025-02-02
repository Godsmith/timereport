use argparse::consume_after_target;
use argparse::consume_bool;
use chrono::prelude::*;
use chrono::TimeDelta;
use std::collections::HashMap;
use std::fmt::Debug;
use std::fs;
use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::path::Path;
mod traits;
use traits::Parsable;
mod argparse;
mod day;
pub mod mockopen;
// Rust note: need to do pub table here since it is used in the binary crate main.rs
pub mod table;
mod timedelta;
use day::Day;

enum ParsedDay {
    Day(Day),
    ParseError(String),
}

impl Debug for ParsedDay {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Day(arg0) => f.debug_tuple("Day").field(arg0).finish(),
            Self::ParseError(arg0) => f.debug_tuple("ParseError").field(arg0).finish(),
        }
    }
}

fn find_date(args: &Vec<String>) -> Option<NaiveDate> {
    args.iter()
        .find_map(|s| NaiveDate::parse_from_str(s, "%Y-%m-%d").ok())
}

pub fn parse_date(text: &str) -> Result<NaiveDateTime, String> {
    let today = Local::now();
    let today_string = today.format("%Y-%m-%d").to_string();
    let time_string = format!("{} {}", today_string, text);
    match NaiveDateTime::parse_from_str(&time_string, "%Y-%m-%d %H:%M") {
        Ok(dt) => Ok(dt),
        Err(e) => {
            Err(format!("Could not parse date string '{}'. Error: '{}'", text, e).to_string())
        }
    }
}

fn parse_args(args: Vec<String>) -> (ParsedDay, Vec<String>) {
    let (start, args_after_start) = consume_after_target("start", args);
    let start = match start {
        Ok(option) => match option {
            None => None,
            Some(text) => match parse_date(&text) {
                Ok(dt) => Some(dt),
                Err(e) => return (ParsedDay::ParseError(e), args_after_start),
            },
        },
        Err(error) => return (ParsedDay::ParseError(error), args_after_start),
    };

    let (stop, args_after_stop) = consume_after_target("stop", args_after_start);
    let stop = match stop {
        Ok(option) => match option {
            None => None,
            Some(text) => match parse_date(&text) {
                Ok(dt) => Some(dt),
                Err(e) => return (ParsedDay::ParseError(e), args_after_stop),
            },
        },
        Err(error) => return (ParsedDay::ParseError(error), args_after_stop),
    };

    let (lunch, args_after_lunch) = consume_after_target("lunch", args_after_stop);
    let lunch = match lunch {
        Ok(option) => match option {
            None => None,
            Some(text) => match TimeDelta::from_str(&text) {
                Ok(dt) => Some(dt),
                Err(e) => return (ParsedDay::ParseError(e), args_after_lunch),
            },
        },
        Err(error) => return (ParsedDay::ParseError(error), args_after_lunch),
    };

    // TODO: consume here as well
    let date = find_date(&args_after_lunch);
    let date = match date {
        None => Local::now().date_naive(),
        Some(date) => date,
    };

    (
        ParsedDay::Day(Day {
            date: date,
            start: start,
            stop: stop,
            lunch: lunch,
        }),
        args_after_lunch,
    )
}

fn create_empty_json_file(path: &Path) {
    let mut file =
        fs::File::create(path).expect(&format!("Failed to create file {}", path.to_string_lossy()));
    file.write_all(b"{}").expect(&format!(
        "Could not write to file {}",
        path.to_str().unwrap()
    ));
}

fn load_days(path: &Path) -> HashMap<NaiveDate, Day> {
    if fs::metadata(path).is_err() {
        create_empty_json_file(path);
    }
    let mut file = File::open(path).expect("Failed to open days.json");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect(&format!("Failed to read {}", path.to_string_lossy()));
    serde_json::from_str(&contents).expect("Failed to parse JSON")
}

fn save_days(days: &HashMap<NaiveDate, Day>, path: &Path) {
    let json_string = serde_json::to_string_pretty(&days).unwrap();
    match fs::write(path, json_string) {
        Ok(_) => {}
        Err(_) => eprintln!("Error writing to file {}", path.to_string_lossy()),
    }
}

fn show_week_table(updated_day: Option<Day>, path: &Path, show_weekend: bool) -> String {
    let mut days = load_days(path);
    let date = match updated_day {
        None => Local::now().date_naive(),
        Some(day) => {
            let date = day.date;
            match days.get(&date) {
                None => days.insert(date, day),
                Some(old_day) => days.insert(date, old_day.combine(day)),
            };
            save_days(&days, path);
            date
        }
    };
    table::create_week_table(date, days, show_weekend)
}

fn show_week_table_html(path: &Path, show_weekend: bool) -> String {
    let date = Local::now().date_naive();
    let days = load_days(path);
    match table::create_html_week_table(date, days, show_weekend) {
        Ok(_) => "".to_string(),
        Err(error) => format!("Error: '{}'", error.to_string()),
    }
}

pub fn main(args: Vec<String>, path: &Path) -> String {
    let (show_weekend, args_after_show_weekend) = consume_bool("--weekend", args);
    let (show_html, args_after_show_html) = consume_bool("html", args_after_show_weekend);
    let (result_for_arg_after_show, args_after_consuming_show) =
        consume_after_target("show", args_after_show_html);

    let arg_after_show = match result_for_arg_after_show {
        Err(message) => return message,
        Ok(value_after_show) => value_after_show,
    };
    match arg_after_show.as_deref() {
        None => {}
        Some(value) => match value {
            "week" => {
                if show_html {
                    return show_week_table_html(path, show_weekend);
                } else {
                    return show_week_table(None, path, show_weekend);
                }
            }
            _ => return format!("Unknown show command: {}", value),
        },
    };

    let (parsed_day, args_after_parsing_args) = parse_args(args_after_consuming_show);
    println!("{:?}", parsed_day);
    match parsed_day {
        ParsedDay::ParseError(description) => description,
        ParsedDay::Day(day) => show_week_table(Some(day), path, show_weekend),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;
    #[rstest]
    fn parsed_day_debug() {
        let day = Day {
            date: NaiveDate::parse_from_str("2025-02-02", "%Y-%m-%d").expect(""),
            start: Some(
                NaiveDateTime::parse_from_str("2025-02-02 8:00", "%Y-%m-%d %H:%M").expect(""),
            ),
            stop: Some(
                NaiveDateTime::parse_from_str("2025-02-02 17:00", "%Y-%m-%d %H:%M").expect(""),
            ),
            lunch: Some(TimeDelta::from_str("45m").expect("")),
        };
        let parsed_day = ParsedDay::Day(day);
        let debug_text = format!("{:?}", parsed_day);
        assert!(debug_text.contains("2025-02-02"));
        assert!(debug_text.contains("8:00"));
        assert!(debug_text.contains("17:00"));
        assert!(debug_text.contains("2700")); // 2700 s == 45 min
    }

    #[rstest]
    fn parse_date_error() {
        let output = parse_date("2024-01-32");
        assert_eq!(
            output,
            Result::Err(
                "Could not parse date string '2024-01-32'. Error: 'input contains invalid characters'"
                    .to_string()
            )
        );
    }
}
