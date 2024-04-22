use chrono::prelude::*;
use chrono::TimeDelta;
// use serde::{Deserialize, Serialize};
// use std::collections::HashMap;
use std::env;
use std::io;
use tabled::{
    builder::Builder,
    settings::{object::Rows, Alignment, Modify, Style},
};

struct Day {
    start: Option<NaiveDateTime>,
    stop: Option<NaiveDateTime>,
    lunch: Option<TimeDelta>,
}

enum ParsedDay {
    Day(Day),
    ParseError(String),
}

fn days_in_current_week() -> Vec<String> {
    let offset = Local::now().weekday().num_days_from_monday();
    let timedelta_to_last_monday = TimeDelta::try_days(-i64::from(offset)).unwrap();
    let date_of_last_monday = Local::now() + timedelta_to_last_monday;
    let timedeltas = (0..=4)
        .map(|i| TimeDelta::try_days(i).unwrap())
        .map(|d| date_of_last_monday + d)
        .map(|datetime| {
            datetime
                .to_rfc3339()
                .split_terminator("T")
                .next()
                .unwrap()
                .to_string()
        });
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

fn parse_args(args: &Vec<String>) -> ParsedDay {
    let start = match parse_came(args) {
        // Ok(option) => match option {
        //     Some(text) => match NaiveDateTime::parse_from_str(&text, "%H:%M") {
        //         Err(_) => None,
        //         Ok(value) => value,
        // },
        Ok(option) => match option {
            None => None,
            Some(text) => {
                println!("{text}");
                match NaiveDateTime::parse_from_str(&text, "%H:%M") {
                    Ok(dt) => Some(dt),
                    Err(e) => {
                        return ParsedDay::ParseError(
                            format!("Could not parse date string {}. Error: {}", text, e)
                                .to_string(),
                        )
                    }
                }
            }
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

fn print_current_week(day: Day) {
    let mut builder = Builder::default();
    builder.push_record(days_in_current_week());
    let table = builder.build().to_string();
    println!("{table}")
}

fn main() {
    let args: Vec<_> = env::args().collect();
    let parsed_day = parse_args(&args);
    match parsed_day {
        ParsedDay::Day(day) => print_current_week(day),
        ParsedDay::ParseError(description) => println!("{description}"),
    }
}
