use chrono::{Datelike, Days, IsoWeek, NaiveDate, Weekday};
use date_validation_types::units::{ValidatedDate, ValidatedYear};
use derive_getters::Getters;
use prettytable::{row, Row};

use crate::{chrono_utils, validated_week_number::ValidatedWeekNumber};

#[derive(Getters, Debug, PartialEq, Eq)]
pub struct WeekCalendarNumber {
    week: IsoWeek,
    monday: NaiveDate,
    sunday: NaiveDate,
}

impl WeekCalendarNumber {
    pub fn new(date: impl Into<NaiveDate>) -> Self {
        let date: NaiveDate = date.into();
        let week = date.week(Weekday::Mon);
        let monday = week.first_day();
        let sunday = week.last_day();
        let week = monday.iso_week();
        Self {
            week,
            monday,
            sunday,
        }
    }

    pub fn new_week_number(number: ValidatedWeekNumber, year: ValidatedYear) -> Option<Self> {
        let year: i32 = u32::from(year) as i32;
        let first_date: NaiveDate = NaiveDate::from_ymd_opt(year, 1, 1).unwrap();

        let mut current_date = WeekCalendarNumber::new(first_date);
        let number = number.into();
        while current_date.week_number() != number {
            let next_monday = current_date.sunday().checked_add_days(Days::new(1))?;
            if next_monday.year() > year {
                return None;
            }
            current_date = WeekCalendarNumber::new(next_monday);
        }
        Some(current_date)
    }

    pub fn today() -> Self {
        chrono_utils::get_today_date().into()
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

    pub fn week_number(&self) -> u32 {
        self.week.week()
    }
}

impl std::fmt::Display for WeekCalendarNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {} {}",
            self.week_number(),
            self.monday(),
            self.sunday()
        )
    }
}

impl From<WeekCalendarNumber> for Row {
    fn from(value: WeekCalendarNumber) -> Self {
        row![value.week_number(), value.monday, value.sunday]
    }
}

impl From<NaiveDate> for WeekCalendarNumber {
    fn from(value: NaiveDate) -> Self {
        let validated_date: ValidatedDate = value.into();
        Self::new(validated_date)
    }
}

#[cfg(test)]
mod testing {
    use super::*;
    #[test]
    fn new_with_calendar_week_and_monday_and_sunday() {
        let actual = WeekCalendarNumber::new(ValidatedDate::from_ymd(2023, 6, 30).unwrap());

        assert_eq!(26, actual.week_number());
        assert_eq!(NaiveDate::from_ymd_opt(2023, 6, 26).unwrap(), actual.monday);
        assert_eq!(NaiveDate::from_ymd_opt(2023, 7, 2).unwrap(), actual.sunday);
    }

    #[test]
    fn new_with_calendar_single_week_1_1_2020() {
        let actual = WeekCalendarNumber::new(ValidatedDate::from_ymd(2020, 1, 1).unwrap());

        assert_eq!(1, actual.week_number());
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

    #[test]
    fn week_number() {
        fn assert_case(
            week_number: ValidatedWeekNumber,
            year: ValidatedYear,
            expected: Option<String>,
        ) {
            let actual = WeekCalendarNumber::new_week_number(week_number, year)
                .as_ref()
                .map(ToString::to_string);
            assert_eq!(
                expected, actual,
                "week_number: {:?}\nyear: {:?}",
                week_number, year
            );
        }
        assert_case(53.into(), 2024.try_into().unwrap(), None);
        assert_case(
            36.into(),
            2024.try_into().unwrap(),
            Some("36 2024-09-02 2024-09-08".to_owned()),
        );
        assert_case(
            52.into(),
            2024.try_into().unwrap(),
            Some("52 2024-12-23 2024-12-29".to_owned()),
        );
    }
}
