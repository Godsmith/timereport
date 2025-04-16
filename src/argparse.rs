use chrono::{Datelike, NaiveDate, Weekday};

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
pub fn consume_two_after_target(
    target: &str,
    args: Vec<String>,
) -> (Result<Option<(String, String)>, String>, Vec<String>) {
    // First, find the position of the target without holding a borrow
    let pos = args.iter().position(|s| s == target);

    match pos {
        None => (Ok(None), args),
        Some(i) => {
            if i + 2 >= args.len() {
                (Err(format!("Not enough arguments after {}", target)), args)
            } else {
                // Extract the two values after the target
                let s1 = args[i + 1].clone();
                let s2 = args[i + 2].clone();

                // Create a new vector excluding the target and the next two elements
                let modified: Vec<String> = args
                    .into_iter() // Take ownership of args
                    .enumerate()
                    .filter_map(|(idx, s)| {
                        if idx != i && idx != i + 1 && idx != i + 2 {
                            Some(s)
                        } else {
                            None
                        }
                    })
                    .collect();

                (Ok(Some((s1, s2))), modified)
            }
        }
    }
}
pub fn consume_dates(args: Vec<String>, today: NaiveDate) -> (Vec<NaiveDate>, Vec<String>) {
    let mut dates = Vec::new(); // To store the collected dates
    let mut remaining_args = args; // Start with the full input arguments

    loop {
        // Call consume_date with the remaining arguments
        let (date, new_remaining_args) = consume_date(remaining_args, today);

        match date {
            Some(d) => {
                // If a date was found, add it to the dates vector
                dates.push(d);
                // Update the remaining arguments for the next iteration
                remaining_args = new_remaining_args;
            }
            None => {
                // If no date was found, return the collected dates and remaining arguments
                return (dates, new_remaining_args);
            }
        }
    }
}

fn consume_date(args: Vec<String>, today: NaiveDate) -> (Option<NaiveDate>, Vec<String>) {
    for (index, arg) in args.iter().enumerate() {
        match date_from_arg(arg, today) {
            Some(date) => {
                // Create new vector without the weekday string
                let modified = args
                    .into_iter()
                    .enumerate()
                    .filter_map(|(i, s)| (i != index).then_some(s))
                    .collect();

                return (Some(date), modified);
            }
            None => (),
        }
    }
    (None, args)
}

fn date_from_arg(arg: &String, today: NaiveDate) -> Option<NaiveDate> {
    if arg.to_lowercase() == "yesterday" {
        let yesterday = today
            .pred_opt()
            .expect("the day is not the first day in history");
        return Some(yesterday);
    }

    let weekdays = [
        "monday",
        "tuesday",
        "wednesday",
        "thursday",
        "friday",
        "saturday",
        "sunday",
    ];
    match weekdays.iter().position(|&x| x == arg.to_lowercase()) {
        Some(position) => {
            let weekday = Weekday::try_from(position as u8).unwrap();
            let days_since_monday = today.weekday().num_days_from_monday();
            let target_days_since_monday = weekday.num_days_from_monday();
            let date = today
                - chrono::Duration::try_days(days_since_monday as i64).expect("must be 0-6")
                + chrono::Duration::try_days(target_days_since_monday as i64).expect("must be 0-6");
            return Some(date);
        }
        None => (),
    }
    // If no weekday found, try parsing from date
    match NaiveDate::parse_from_str(arg, "%Y-%m-%d") {
        Ok(date) => return Some(date),
        Err(_) => (),
    }
    return None;
}
