use chrono::{Datelike, Local, NaiveDate, Weekday};
use date_validation_types::{ValidatedDate, ValidatedDay, ValidatedMonth, ValidatedYear};
use derive_getters::Getters;
use prettytable::Row;

#[derive(Getters)]
pub struct WeekWithCalendarNumber {
    week: u32,
    monday: NaiveDate,
    sunday: NaiveDate,
}

impl WeekWithCalendarNumber {
    pub fn new(date: ValidatedDate) -> Self {
        let date: NaiveDate = date.into();
        let first_day_of_year = NaiveDate::from_ymd_opt(date.year(), 1, 1).unwrap();
        let week = date.week(Weekday::Mon);
        let monday = week.first_day();
        let sunday = week.last_day();
        let week = monday - first_day_of_year;
        let week = week.num_weeks() as u32 + 1;
        Self {
            week,
            monday,
            sunday,
        }
    }

    pub fn today() -> Self {
        let today: NaiveDate = Local::now().naive_local().date();
        today.into()
    }
}

use prettytable::row;
impl From<WeekWithCalendarNumber> for Row {
    fn from(value: WeekWithCalendarNumber) -> Self {
        row![value.week, value.monday, value.sunday]
    }
}
impl From<NaiveDate> for WeekWithCalendarNumber {
    fn from(value: NaiveDate) -> Self {
        let year = value.year() as u32;
        let year: ValidatedYear = year.try_into().unwrap();
        let month: ValidatedMonth = value.month().try_into().unwrap();
        let day: ValidatedDay = value.day().try_into().unwrap();
        Self::new(ValidatedDate::new(year, month, day).unwrap())
    }
}

#[cfg(test)]
mod testing {
    use super::*;
    #[test]
    fn new_with_calendar_week_and_monday_and_sunday() {
        let actual = WeekWithCalendarNumber::new(ValidatedDate::from_ymd(2023, 6, 30).unwrap());

        assert_eq!(26, actual.week);
        assert_eq!(NaiveDate::from_ymd_opt(2023, 6, 26).unwrap(), actual.monday);
        assert_eq!(NaiveDate::from_ymd_opt(2023, 7, 2).unwrap(), actual.sunday);
    }
}
