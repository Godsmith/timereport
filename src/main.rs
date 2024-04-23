use chrono::prelude::*;
use chrono::TimeDelta;
use serde::{Deserialize, Deserializer, Serialize};
use std::collections::HashMap;
// use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::Read;
use tabled::builder::Builder;

#[derive(Deserialize)]
struct Day {
    start: Option<NaiveDateTime>,
    stop: Option<NaiveDateTime>,
    #[serde(deserialize_with = "deserialize_timedelta")]
    lunch: Option<TimeDelta>,
}

struct SerializableDay {
    start: Option<String>,
    stop: Option<String>,
    lunch: Option<String>,
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

fn days_in_current_week() -> Vec<NaiveDate> {
    let offset = Local::now().weekday().num_days_from_monday();
    let timedelta_to_last_monday = TimeDelta::try_days(-i64::from(offset)).unwrap();
    let date_of_last_monday = Local::now() + timedelta_to_last_monday;
    let timedeltas = (0..=4)
        .map(|i| TimeDelta::try_days(i).unwrap())
        .map(|d| date_of_last_monday + d)
        .map(|datetime| datetime.date_naive());
    timedeltas.collect::<Vec<_>>()
}

fn parse_came(args: &Vec<String>) -> Result<Option<String>, String> {
    match args.iter().position(|s| s == "came") {
        None => Ok(None),
        Some(index) => match args.get(index + 1) {
            Some(value) => Ok(Some(value.to_string())),
            None => Err("no argument after 'came'".to_string()),
        },
    }
}

fn parse_time(text: &str) -> Result<NaiveDateTime, String> {
    let today = Local::now();
    let today_string = today.format("%Y-%m-%d").to_string();
    let time_string = format!("{} {}", today_string, text);
    match NaiveDateTime::parse_from_str(&time_string, "%Y-%m-%d %H:%M") {
        Ok(dt) => Ok(dt),
        Err(e) => Err(format!("Could not parse date string {}. Error: {}", text, e).to_string()),
    }
}

fn parse_args(args: &Vec<String>) -> ParsedDay {
    let start = match parse_came(args) {
        Ok(option) => match option {
            None => None,
            Some(text) => match parse_time(&text) {
                Ok(dt) => Some(dt),
                Err(e) => return ParsedDay::ParseError(e),
            },
        },
        Err(error) => return ParsedDay::ParseError(error),
    };

    match start {
        Some(start) => ParsedDay::Day(Day {
            start: Some(start),
            stop: None,
            lunch: None,
        }),
        None => ParsedDay::ParseError("start is None".to_string()),
    }
}

fn print_current_week(days: HashMap<NaiveDate, Day>) {
    let mut builder = Builder::default();
    let week_days = days_in_current_week();
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
    let mut start_row = vec!["came".to_string()];
    start_row.extend(starts);
    builder.push_record(start_row);
    let table = builder.build().to_string();
    println!("{table}")
}

fn load_days(path: &str) -> HashMap<NaiveDate, Day> {
    let mut file = File::open(path).expect("Failed to open days.json");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect(&format!("Failed to read {}", path));
    serde_json::from_str(&contents).expect("Failed to parse JSON")
}

fn main() {
    let mut days = load_days("days.json");
    let args: Vec<_> = env::args().collect();
    let parsed_day = parse_args(&args);
    let local_date = Local::today();
    let naive_date = local_date.naive_local();
    match parsed_day {
        ParsedDay::ParseError(description) => println!("{description}"),
        ParsedDay::Day(day) => {
            days.insert(naive_date, day);
            print_current_week(days);
        }
    }
}
