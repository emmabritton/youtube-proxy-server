use chrono::{DateTime, Utc};
use chrono::prelude::*;

pub trait DateExt {
    fn into_start_of_month(self) -> DateTime<Utc>;
    fn into_end_of_month(self) -> DateTime<Utc>;
    fn minus_months(self, count: u32) -> Option<DateTime<Utc>>;
    fn into_month_surround(self) -> (DateTime<Utc>, DateTime<Utc>);
}

impl DateExt for DateTime<Utc> {
    fn into_start_of_month(self) -> DateTime<Utc> {
        self.with_day(1).unwrap()
            .with_hour(0).unwrap()
            .with_minute(0).unwrap()
            .with_second(0).unwrap()
            .with_nanosecond(0).unwrap()
    }

    fn into_end_of_month(self) -> DateTime<Utc> {
        self.with_day(days_in_month(self.month() as u8, self.year() as u32) as u32).unwrap()
            .with_hour(23).unwrap()
            .with_minute(59).unwrap()
            .with_second(59).unwrap()
            .with_nanosecond(999_999_999).unwrap()
    }

    fn minus_months(self, count: u32) -> Option<DateTime<Utc>> {
        let years = count / 12;
        let months = count - (years * 12);
        self.with_year(self.year() - years as i32)?
            .with_month(self.month() - months)
    }

    fn into_month_surround(self) -> (DateTime<Utc>, DateTime<Utc>) {
        let start = self.into_start_of_month();
        let end = start.clone().into_end_of_month();
        (start, end)
    }
}

pub fn get_start_and_end_for_previous_month(from: DateTime<Utc>, month_count: u32) -> (DateTime<Utc>, DateTime<Utc>) {
    from.minus_months(month_count).unwrap().into_month_surround()
}

/// Returns days in a month (accounting for leap years)
/// Month are 1 based (i.e Jan = 1, Dec = 12)
pub fn days_in_month(month: u8, year: u32) -> u8 {
    match month {
        2 => {
            if is_leap_year(year) {
                29
            } else {
                28
            }
        }
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        _ => panic!("Invalid month: {}", month)
    }
}

pub fn is_leap_year(year: u32) -> bool {
    return (year % 4 == 0) && (year % 100 != 0 || year % 400 == 0);
}

#[cfg(test)]
mod test {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn test_days_in_month() {
        assert_eq!(days_in_month(1, 1900), 31, "Jan has 31");
        assert_eq!(days_in_month(2, 1900), 28, "Feb has 28 on non leap years");
        assert_eq!(days_in_month(3, 1900), 31, "Mar has 31");
        assert_eq!(days_in_month(4, 1900), 30, "Apr has 30");
        assert_eq!(days_in_month(5, 1900), 31, "May has 31");
        assert_eq!(days_in_month(6, 1900), 30, "Jun has 30");
        assert_eq!(days_in_month(7, 1900), 31, "Jul has 31");
        assert_eq!(days_in_month(8, 1900), 31, "Aug has 31");
        assert_eq!(days_in_month(9, 1900), 30, "Sep has 30");
        assert_eq!(days_in_month(10, 1900), 31, "Oct has 31");
        assert_eq!(days_in_month(11, 1900), 30, "Nov has 30");
        assert_eq!(days_in_month(12, 1900), 31, "Dec has 31");
        assert_eq!(days_in_month(1, 2000), 31, "Jan always has 31");
        assert_eq!(days_in_month(2, 2000), 29, "Feb has 29 on leap years");
    }

    #[test]
    fn test_leap_year() {
        assert!(!is_leap_year(1900), "is not leap year");
        assert!(!is_leap_year(2100), "is not leap year");
        assert!(!is_leap_year(2101), "is not leap year");
        assert!(is_leap_year(2000), "is leap year");
        assert!(is_leap_year(2016), "is leap year");
    }

    #[test]
    fn test_start_of_month() {
        //GIVEN
        let test_date: DateTime<Utc> = DateTime::from_str("2020-01-01T20:30:12Z").unwrap();
        let mut already_at_start = test_date.clone().into_start_of_month();
        let mut fifth_day = test_date.clone().with_day(5).unwrap();
        let mut end_of_month = test_date.clone().into_end_of_month();
        let mut feb = DateTime::from_str("2020-02-22T04:34:45Z").unwrap();

        //WHEN
        already_at_start = already_at_start.into_start_of_month();
        fifth_day = fifth_day.into_start_of_month();
        end_of_month = end_of_month.into_start_of_month();
        feb = feb.into_start_of_month();

        //THEN
        assert_eq!(already_at_start.year(), 2020);
        assert_eq!(already_at_start.month(), 1);
        assert_eq!(already_at_start.day(), 1);
        assert_eq!(already_at_start, fifth_day);
        assert_eq!(already_at_start, end_of_month);
        assert_eq!(feb.year(), 2020);
        assert_eq!(feb.month(), 2);
        assert_eq!(feb.day(), 1);
    }

    #[test]
    fn test_end_of_month() {
        //GIVEN
        let test_date: DateTime<Utc> = DateTime::from_str("2020-01-01T20:30:12Z").unwrap();
        let mut start_of_month = test_date.clone().into_start_of_month();
        let mut fifth_day = test_date.clone().with_day(5).unwrap();
        let mut already_at_end = test_date.clone().into_end_of_month();
        let mut feb = DateTime::from_str("2020-02-22T04:34:45Z").unwrap();

        //WHEN
        already_at_end = already_at_end.into_end_of_month();
        fifth_day = fifth_day.into_end_of_month();
        start_of_month = start_of_month.into_end_of_month();
        feb = feb.into_end_of_month();

        //THEN
        assert_eq!(already_at_end.year(), 2020);
        assert_eq!(already_at_end.month(), 1);
        assert_eq!(already_at_end.day(), 31);
        assert_eq!(already_at_end, fifth_day);
        assert_eq!(already_at_end, start_of_month);
        assert_eq!(feb.year(), 2020);
        assert_eq!(feb.month(), 2);
        assert_eq!(feb.day(), 29);
    }

    #[test]
    fn test_start_end_date_calculation_this_month() {
        //GIVEN
        let time: DateTime<Utc> = DateTime::from_str("2020-01-15T17:20:12Z").unwrap();

        //WHEN
        let (start, end) = time.into_month_surround();

        //THEN
        assert_eq!(start.year(), 2020);
        assert_eq!(start.month(), 1);
        assert_eq!(start.day(), 1);
        assert_eq!(start.hour(), 0);
        assert_eq!(start.minute(), 0);
        assert_eq!(start.second(), 0);
        assert_eq!(end.year(), 2020);
        assert_eq!(end.month(), 1);
        assert_eq!(end.day(), 31);
        assert_eq!(end.hour(), 23);
        assert_eq!(end.minute(), 59);
        assert_eq!(end.second(), 59);
    }

    #[test]
    fn test_start_end_date_calculation_previous_month() {
        //GIVEN
        let time: DateTime<Utc> = DateTime::from_str("2019-06-15T17:20:12Z").unwrap();

        //WHEN
        let (start, end) = time.into_month_surround();

        //THEN
        assert_eq!(start.year(), 2019);
        assert_eq!(start.month(), 6);
        assert_eq!(start.day(), 1);
        assert_eq!(start.hour(), 0);
        assert_eq!(start.minute(), 0);
        assert_eq!(start.second(), 0);
        assert_eq!(end.year(), 2019);
        assert_eq!(end.month(), 6);
        assert_eq!(end.day(), 30);
        assert_eq!(end.hour(), 23);
        assert_eq!(end.minute(), 59);
        assert_eq!(end.second(), 59);
    }
}