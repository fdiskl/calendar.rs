use chrono::{Datelike, NaiveDate, Weekday};

// returns:
// 1. first day number (Mon-Sat) [0;6]
// 2. amount of days in a month
pub fn month_info(year: i32, month: u32) -> (i32, i32) {
    let first_day = NaiveDate::from_ymd_opt(year, month, 1).expect("Invalid date");

    let next_month = if month == 12 { 1 } else { month + 1 };
    let next_year = if month == 12 { year + 1 } else { year };
    let first_day_next_month = NaiveDate::from_ymd_opt(next_year, next_month, 1).unwrap();
    let last_day = first_day_next_month.pred_opt().unwrap();

    let days_in_month = last_day.day();

    let n = match first_day.weekday() {
        Weekday::Mon => 0,
        Weekday::Tue => 1,
        Weekday::Wed => 2,
        Weekday::Thu => 3,
        Weekday::Fri => 4,
        Weekday::Sat => 5,
        Weekday::Sun => 6,
    };

    (n, days_in_month as i32)
}
