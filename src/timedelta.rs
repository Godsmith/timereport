use crate::traits::Parsable;
use chrono::TimeDelta;
use regex::Regex;

impl Parsable for TimeDelta {
    fn from_str(text: &str) -> Result<Self, String> {
        let re = Regex::new(r"(\d+):?(\d*)").unwrap();
        let (hours, minutes) = match re.captures(text) {
            None => return Err(format!("Could not parse timedelta string {}.", text)),
            Some(captures) => {
                let (_, b): (&str, [&str; 2]) = captures.extract();
                (b[0], b[1])
            }
        };
        let mut seconds = 0;
        match hours.parse::<i64>() {
            Err(_) => return Err(format!("Could not parse timedelta string {}.", text)),
            Ok(hours) => seconds += hours * 3600,
        }
        match minutes.parse::<i64>() {
            Err(_) => return Err(format!("Could not parse timedelta string {}.", text)),
            Ok(minutes) => seconds += minutes * 60,
        }
        match TimeDelta::new(seconds, 0) {
            Some(timedelta) => Ok(timedelta),
            None => return Err(format!("Could not parse timedelta string {}.", text)),
        }
    }
}
