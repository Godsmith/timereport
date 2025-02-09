use chrono::{prelude::*, Duration};
pub fn last_day_of_month(date: NaiveDate) -> NaiveDate {
    let year = date.year();
    let month = date.month();

    let last_day = if month == 12 {
        NaiveDate::from_ymd_opt(year + 1, 1, 1).unwrap()
    } else {
        NaiveDate::from_ymd_opt(year, month + 1, 1).unwrap()
    }
    .pred_opt()
    .unwrap();

    last_day
}

pub fn one_date_per_week(first_date: NaiveDate, last_date: NaiveDate) -> Vec<NaiveDate> {
    // Generate dates using an iterator
    (0..)
        .map(|weeks| first_date + Duration::try_weeks(weeks).expect("should be a low number")) // Add weeks to first_date
        .take_while(|&date| date <= last_date) // Stop when date exceeds last_date
        .collect() // Collect into Vec<NaiveDate>
}
