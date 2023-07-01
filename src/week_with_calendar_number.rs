use chrono::{Datelike, Local, NaiveDate, Weekday};
use date_validation_types::{ValidatedDate, ValidatedDay, ValidatedMonth, ValidatedYear};
use derive_getters::Getters;
use prettytable::{row, Row};

#[derive(Getters, Debug)]
pub struct WeekWithCalendarNumber {
    week: u32,
    monday: NaiveDate,
    sunday: NaiveDate,
}

impl WeekWithCalendarNumber {
    pub fn new(date: impl Into<NaiveDate>) -> Self {
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

    pub fn all_new_in_month(date: impl Into<NaiveDate>) -> Vec<Self> {
        let date: NaiveDate = date.into();
        dbg!(date);
        let last_date_of_month = Self::get_last_day_current_month(date);
        let month = date.month();
        let first_week = NaiveDate::from_ymd_opt(date.year(), month, 1).unwrap();
        let mut current_week = first_week.week(Weekday::Mon).first_day();
        let mut weeks: Vec<Self> = Vec::with_capacity(6);

        while current_week <= last_date_of_month {
            weeks.push(Self::new(current_week));
            current_week = current_week.week(Weekday::Mon).last_day() + chrono::Duration::days(1);
        }

        weeks
    }

    fn get_last_day_current_month(date: NaiveDate) -> NaiveDate {
        let (next_month, next_year) = {
            let mut next_year = date.year();
            let next_month = if date.month() == 12 {
                next_year += 1;
                1
            } else {
                date.month() + 1
            };

            (next_month, next_year)
        };

        NaiveDate::from_ymd_opt(next_year, next_month, 1).unwrap() - chrono::Duration::days(1)
    }
}

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
    #[test]
    fn new_with_calendar_single_week_1_1_2020() {
        let actual = WeekWithCalendarNumber::new(ValidatedDate::from_ymd(2020, 1, 1).unwrap());

        assert_eq!(1, actual.week);
        assert_eq!(
            NaiveDate::from_ymd_opt(2019, 12, 30).unwrap(),
            actual.monday
        );
        assert_eq!(NaiveDate::from_ymd_opt(2020, 1, 5).unwrap(), actual.sunday);
    }
    #[test]
    fn new_within_monts() {
        let actual =
            WeekWithCalendarNumber::all_new_in_month(ValidatedDate::from_ymd(2023, 6, 30).unwrap());
        insta::assert_debug_snapshot!(actual);
    }
    #[test]
    fn new_within_months_from_earlier_year() {
        let actual =
            WeekWithCalendarNumber::all_new_in_month(ValidatedDate::from_ymd(2023, 1, 1).unwrap());
        insta::assert_debug_snapshot!(actual);
    }
    #[test]
    fn new_within_months_at_end_of_year_2023() {
        let actual = WeekWithCalendarNumber::all_new_in_month(
            NaiveDate::from_ymd_opt(2023, 12, 29).unwrap(),
        );
        insta::assert_debug_snapshot!(actual);
    }
    #[test]
    fn new_within_months_at_end_of_year_2022() {
        let actual = WeekWithCalendarNumber::all_new_in_month(
            NaiveDate::from_ymd_opt(2022, 12, 10).unwrap(),
        );
        insta::assert_debug_snapshot!(actual);
    }
}
