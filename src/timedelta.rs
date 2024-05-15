use crate::traits::Parsable;
use chrono::TimeDelta;
use regex::Regex;

fn to_hours_and_minutes(text: &str) -> Result<(&str, &str), String> {
    let re = Regex::new(r"(\d+):?(\d\d)").unwrap();
    if let Some(captures) = re.captures(text) {
        let (_, groups): (&str, [&str; 2]) = captures.extract();
        return Ok((groups[0], groups[1]));
    }
    let re2 = Regex::new(r"(\d+)m").unwrap();
    if let Some(captures) = re2.captures(text) {
        let (_, groups): (&str, [&str; 1]) = captures.extract();
        return Ok((&"0", groups[0]));
    }
    return Err(format!("Could not parse timedelta string '{}'.", text));
}

impl Parsable for TimeDelta {
    fn from_str(text: &str) -> Result<Self, String> {
        let (hours, minutes) = to_hours_and_minutes(text)?;
        // Only panics when the minutes or hours strings are not integers, which should
        // not happen because then they wouldn't have matched the regex
        let seconds = hours.parse::<i64>().unwrap() * 3600 + minutes.parse::<i64>().unwrap() * 60;
        // Only panics when out of bounds, so should be safe
        Ok(TimeDelta::new(seconds, 0).unwrap())
    }
}
