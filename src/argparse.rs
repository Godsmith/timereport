use std::convert::TryFrom;

use chrono::{Datelike, Local, NaiveDate, Weekday};

pub fn consume_bool(target: &str, args: Vec<String>) -> (bool, Vec<String>) {
    // Check if the target string exists in the vector
    let exists = args.iter().any(|s| s == target);

    // Create a new vector with the target string removed
    let filtered_args: Vec<String> = args.into_iter().filter(|s| *s != target).collect();

    // Return the tuple
    (exists, filtered_args)
}

pub fn consume_after_target(
    target: &str,
    args: Vec<String>,
) -> (Result<Option<String>, String>, Vec<String>) {
    args.iter().position(|s| s == target).map_or_else(
        || (Ok(None), args.to_vec()),
        |i| {
            if i >= args.len() - 1 {
                (Err(format!("No argument after {}", target)), args.to_vec())
            } else {
                let modified = args
                    .iter()
                    .enumerate()
                    .filter_map(|(idx, s)| {
                        if idx != i && idx != i + 1 {
                            Some(s.clone())
                        } else {
                            None
                        }
                    })
                    .collect();

                (Ok(Some(args[i + 1].clone())), modified)
            }
        },
    )
}
pub fn consume_date(args: Vec<String>) -> (Option<NaiveDate>, Vec<String>) {
    // List of valid weekday names
    let weekdays = [
        "monday",
        "tuesday",
        "wednesday",
        "thursday",
        "friday",
        "saturday",
        "sunday",
    ];

    // Find first weekday name and its index
    let found_weekday = args.iter().enumerate().find_map(|(i, s)| {
        weekdays
            .iter()
            .position(|&w| w == s.to_lowercase())
            .map(|w| (i, Weekday::try_from(w as u8).unwrap()))
    });

    if let Some((index, weekday)) = found_weekday {
        // Calculate the date of the given weekday in the current week
        let today = Local::now().date_naive();
        let days_since_monday = today.weekday().num_days_from_monday();
        let target_days_since_monday = weekday.num_days_from_monday();
        let date = today
            - chrono::Duration::try_days(days_since_monday as i64).expect("must be 0-6")
            + chrono::Duration::try_days(target_days_since_monday as i64).expect("must be 0-6");

        // Create new vector without the weekday string
        let modified = args
            .into_iter()
            .enumerate()
            .filter_map(|(i, s)| (i != index).then_some(s))
            .collect();

        return (Some(date), modified);
    }

    // Fall back to parsing NaiveDate if no weekday is found
    let found_date = args.iter().enumerate().find_map(|(i, s)| {
        NaiveDate::parse_from_str(s, "%Y-%m-%d")
            .ok()
            .map(|d| (i, d))
    });

    match found_date {
        Some((index, date)) => {
            // Create new vector without the date string
            let modified = args
                .into_iter()
                .enumerate()
                .filter_map(|(i, s)| (i != index).then_some(s))
                .collect();

            (Some(date), modified)
        }
        None => (None, args),
    }
}
