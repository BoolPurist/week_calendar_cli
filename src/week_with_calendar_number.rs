use chrono::{Datelike, Local, NaiveDate, Weekday};
use date_validation_types::{ValidatedDate, ValidatedDay, ValidatedMonth, ValidatedYear};
use derive_getters::Getters;
use prettytable::{row, Row};

#[derive(Getters, Debug)]
pub struct WeekCalendarNumber {
    week: u32,
    monday: NaiveDate,
    sunday: NaiveDate,
}

impl WeekCalendarNumber {
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
        let last_date_of_month = Self::last_day_current_month(date);
        let month = date.month();
        let first_week = NaiveDate::from_ymd_opt(date.year(), month, 1)
            .expect("year and month come from a valid date. Every month has its 1. day.");
        let current_week = first_week.week(Weekday::Mon).first_day();

        Self::get_all_weeks_until_monday_old_enough(current_week, last_date_of_month)
    }

    pub fn all_new_in_year(year: impl Into<ValidatedYear>) -> Vec<Self> {
        let year: ValidatedYear = year.into();
        let year = u32::from(year) as i32;
        let last_date_of_year = NaiveDate::from_ymd_opt(year, 12, 31).unwrap();

        let first_week = NaiveDate::from_ymd_opt(year, 1, 1)
            .expect("year and month come from a valid date. Every month has its 1. day.");
        let current_week = first_week.week(Weekday::Mon).first_day();

        Self::get_all_weeks_until_monday_old_enough(current_week, last_date_of_year)
    }

    fn get_all_weeks_until_monday_old_enough(monday: NaiveDate, threshold: NaiveDate) -> Vec<Self> {
        let mut weeks: Vec<Self> = Default::default();
        let mut current_week = monday;

        while current_week <= threshold {
            weeks.push(Self::new(current_week));
            current_week = current_week.week(Weekday::Mon).last_day() + chrono::Duration::days(1);
        }

        weeks
    }

    /// # Summary
    ///
    /// Gets last day of the month of a given date.
    ///
    /// # Example
    ///
    /// 2020-01-14 => 2020-01-31
    fn last_day_current_month(date: NaiveDate) -> NaiveDate {
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

impl From<WeekCalendarNumber> for Row {
    fn from(value: WeekCalendarNumber) -> Self {
        row![value.week, value.monday, value.sunday]
    }
}
impl From<NaiveDate> for WeekCalendarNumber {
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
        let actual = WeekCalendarNumber::new(ValidatedDate::from_ymd(2023, 6, 30).unwrap());

        assert_eq!(26, actual.week);
        assert_eq!(NaiveDate::from_ymd_opt(2023, 6, 26).unwrap(), actual.monday);
        assert_eq!(NaiveDate::from_ymd_opt(2023, 7, 2).unwrap(), actual.sunday);
    }
    #[test]
    fn new_with_calendar_single_week_1_1_2020() {
        let actual = WeekCalendarNumber::new(ValidatedDate::from_ymd(2020, 1, 1).unwrap());

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
            WeekCalendarNumber::all_new_in_month(ValidatedDate::from_ymd(2023, 6, 30).unwrap());
        insta::assert_debug_snapshot!(actual);
    }
    #[test]
    fn new_within_months_from_earlier_year() {
        let actual =
            WeekCalendarNumber::all_new_in_month(ValidatedDate::from_ymd(2023, 1, 1).unwrap());
        insta::assert_debug_snapshot!(actual);
    }
    #[test]
    fn new_within_months_at_end_of_year_2023() {
        let actual =
            WeekCalendarNumber::all_new_in_month(NaiveDate::from_ymd_opt(2023, 12, 29).unwrap());
        insta::assert_debug_snapshot!(actual);
    }
    #[test]
    fn new_within_months_at_end_of_year_2022() {
        let actual =
            WeekCalendarNumber::all_new_in_month(NaiveDate::from_ymd_opt(2022, 12, 10).unwrap());
        insta::assert_debug_snapshot!(actual);
    }
    #[test]
    fn last_day_current_month() {
        let current_day = NaiveDate::from_ymd_opt(2020, 1, 14).unwrap();
        let last_day_of_current_month = WeekCalendarNumber::last_day_current_month(current_day);
        let expected = NaiveDate::from_ymd_opt(2020, 1, 31).unwrap();
        assert_eq!(expected, last_day_of_current_month);
    }
    #[test]
    fn all_weeks_in_year_1992() {
        let date: ValidatedYear = 1992.try_into().unwrap();
        let result = WeekCalendarNumber::all_new_in_year(date);
        insta::assert_debug_snapshot!(result);
    }
}
