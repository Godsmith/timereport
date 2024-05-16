use chrono::prelude::*;
use chrono::TimeDelta;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::path::Path;
use tabled::builder::Builder;
mod traits;
use traits::Parsable;
mod timedelta;

#[derive(Serialize, Deserialize)]
struct Day {
    date: NaiveDate,
    start: Option<NaiveDateTime>,
    stop: Option<NaiveDateTime>,
    #[serde(deserialize_with = "deserialize_timedelta")]
    #[serde(serialize_with = "serialize_timedelta")]
    lunch: Option<TimeDelta>,
}

struct SerializableDay {
    start: Option<String>,
    stop: Option<String>,
    lunch: Option<String>,
}
fn serialize_timedelta<S>(timedelta: &Option<TimeDelta>, serializer: S) -> Result<S::Ok, S::Error>
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
fn deserialize_timedelta<'de, D>(deserializer: D) -> Result<Option<TimeDelta>, D::Error>
where
    D: Deserializer<'de>,
{
    let seconds = i64::deserialize(deserializer)?;
    Ok(Some(TimeDelta::seconds(seconds)))
}

impl Day {
    fn start_string(&self) -> String {
        match self.start {
            Some(time) => time.format("%H:%M").to_string(),
            None => String::new(),
        }
    }
}

enum ParsedDay {
    Day(Day),
    ParseError(String),
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

fn parse_date(text: &str) -> Result<NaiveDateTime, String> {
    let today = Local::now();
    let today_string = today.format("%Y-%m-%d").to_string();
    let time_string = format!("{} {}", today_string, text);
    match NaiveDateTime::parse_from_str(&time_string, "%Y-%m-%d %H:%M") {
        Ok(dt) => Ok(dt),
        Err(e) => Err(format!("Could not parse date string {}. Error: {}", text, e).to_string()),
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

fn create_week_table(date: NaiveDate, days: HashMap<NaiveDate, Day>, show_weekend: bool) -> String {
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

pub fn main(args: Vec<String>, path: &Path) -> String {
    let mut days = load_days(path);
    let parsed_day = parse_args(&args);
    let show_weekend = args.iter().any(|arg| arg == "--weekend");
    match parsed_day {
        ParsedDay::ParseError(description) => description,
        ParsedDay::Day(day) => {
            let date = day.date;
            days.insert(date, day);
            save_days(&days, path);
            create_week_table(date, days, show_weekend)
        }
    }
}
