use chrono::{Datelike, NaiveDate, Weekday};

pub fn middle_day(year: u32) -> Option<Weekday> {
    let is_leap_year = (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0);

    if is_leap_year {
        return None;
    }

    NaiveDate::from_yo_opt(year as i32, 183).map(|date| date.weekday())
}
