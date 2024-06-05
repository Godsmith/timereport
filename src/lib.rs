use chrono::prelude::*;
use chrono::TimeDelta;
use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::path::Path;
mod traits;
use traits::Parsable;
mod day;
mod table;
mod timedelta;
use day::Day;

enum ParsedDay {
    Day(Day),
    ParseError(String),
}

/// Returns the string after a string `part` in a vector.
///
/// - Returns Ok(string) if everything goes well
/// - Returns Ok(None) if the vector does not contain `part`.
/// - Returns Err(message) if the vector contains `part`, but it is the last
/// string in the list.
///
fn find_arg_after(part: &str, args: &Vec<String>) -> Result<Option<String>, String> {
    match args.iter().position(|s| s == part) {
        None => Ok(None),
        Some(index) => match args.get(index + 1) {
            Some(value) => Ok(Some(value.to_string())),
            None => Err(format!("no argument after '{}'", part)),
        },
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

fn parse_args(args: &Vec<String>) -> ParsedDay {
    let start = match find_arg_after("start", args) {
        Ok(option) => match option {
            None => None,
            Some(text) => match parse_date(&text) {
                Ok(dt) => Some(dt),
                Err(e) => return ParsedDay::ParseError(e),
            },
        },
        Err(error) => return ParsedDay::ParseError(error),
    };

    let stop = match find_arg_after("stop", args) {
        Ok(option) => match option {
            None => None,
            Some(text) => match parse_date(&text) {
                Ok(dt) => Some(dt),
                Err(e) => return ParsedDay::ParseError(e),
            },
        },
        Err(error) => return ParsedDay::ParseError(error),
    };

    let lunch = match find_arg_after("lunch", args) {
        Ok(option) => match option {
            None => None,
            Some(text) => match TimeDelta::from_str(&text) {
                Ok(dt) => Some(dt),
                Err(e) => return ParsedDay::ParseError(e),
            },
        },
        Err(error) => return ParsedDay::ParseError(error),
    };

    let date = find_date(args);
    let date = match date {
        None => Local::now().date_naive(),
        Some(date) => date,
    };

    ParsedDay::Day(Day {
        date: date,
        start: start,
        stop: stop,
        lunch: lunch,
    })
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

fn parse_show_command(args: &Vec<String>) -> Result<Option<String>, String> {
    match find_arg_after("show", &args) {
        Ok(arg_or_none) => match arg_or_none {
            None => {
                println!("none returned");
                Ok(None)
            }
            Some(arg) => Ok(Some(arg)),
        },
        Err(message) => return Err(message),
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

pub fn main(args: Vec<String>, path: &Path) -> String {
    let show_weekend = args.iter().any(|arg| arg == "--weekend");

    let value_after_show = match parse_show_command(&args) {
        Err(message) => return message,
        Ok(value_after_show) => value_after_show,
    };
    match value_after_show.as_deref() {
        None => {}
        Some(value) => match value {
            "week" => return show_week_table(None, path, show_weekend),
            _ => return format!("Unknown show command: {}", value),
        },
    };

    let parsed_day = parse_args(&args);
    match parsed_day {
        ParsedDay::ParseError(description) => description,
        ParsedDay::Day(day) => show_week_table(Some(day), path, show_weekend),
    }
}
