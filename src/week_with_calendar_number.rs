use chrono::{Datelike, Days, NaiveDate, Weekday};
use date_validation_types::units::{ValidatedDate, ValidatedYear};
use prettytable::{row, Row};

use crate::{chrono_utils, validated_week_number::ValidatedWeekNumber};

#[derive(Debug, PartialEq, Eq)]
pub struct WeekCalendar {
    week_number: ValidatedWeekNumber,
    monday: NaiveDate,
    sunday: NaiveDate,
}

impl WeekCalendar {
    pub fn week_number(&self) -> u32 {
        self.week_number.into()
    }

    pub fn monday(&self) -> NaiveDate {
        self.monday
    }

    pub fn sunday(&self) -> NaiveDate {
        self.sunday
    }

    pub fn new(date: impl Into<NaiveDate>) -> Self {
        let date: NaiveDate = date.into();
        let week = date.week(Weekday::Mon);
        let monday = week.first_day();
        let sunday = week.last_day();
        let week = ValidatedWeekNumber::new(monday.iso_week().week()).unwrap();
        Self {
            week_number: week,
            monday,
            sunday,
        }
    }

    pub fn new_week_number(
        given_week_number: ValidatedWeekNumber,
        given_year: ValidatedYear,
    ) -> Self {
        const WEEK_NUMBERS_START_WITH_ONE: u32 = 1;

        let mut current_date = create_first_date_as_starting_point(given_year);
        let limit = calc_limit(current_date, given_week_number);

        for _ in WEEK_NUMBERS_START_WITH_ONE..limit {
            current_date = current_date.checked_add_days(Days::new(7)).unwrap();
        }

        return Self::inner_on_week_number(current_date, ValidatedWeekNumber::new(limit).unwrap());

        fn create_first_date_as_starting_point(given_year: ValidatedYear) -> NaiveDate {
            let year: i32 = u32::from(given_year) as i32;
            let mut first_date: NaiveDate = NaiveDate::from_yo_opt(year, 1).unwrap();
            while first_date.iso_week().week() != 1 {
                first_date = first_date.checked_add_days(Days::new(1)).unwrap();
            }

            first_date
        }

        fn calc_limit(first_date: NaiveDate, given_week_number: ValidatedWeekNumber) -> u32 {
            let max_week_number = u32::from(chrono_utils::calc_max_week_number_in_year(
                ValidatedYear::new(first_date.year() as u32)
                    .expect("Years comes from a valid date"),
            ));
            max_week_number.min(given_week_number.into())
        }
    }

    pub fn between_week_numbers(
        start: ValidatedWeekNumber,
        end: ValidatedWeekNumber,
        year: ValidatedYear,
    ) -> Vec<WeekCalendar> {
        debug_assert!(start <= end);

        let mut week_numbers_in_between = Vec::new();

        let first = Self::new_week_number(start, year);
        let mut current_date = first.monday();
        week_numbers_in_between.push(first);

        let (end, start): (u32, u32) = (end.into(), start.into());

        for _next_week_number in start..end {
            current_date = current_date.checked_add_days(Days::new(7)).unwrap();
            week_numbers_in_between.push(WeekCalendar::new(current_date));
        }

        week_numbers_in_between
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

    fn inner_on_week_number(data: NaiveDate, week_number: ValidatedWeekNumber) -> Self {
        let week = data.week(Weekday::Mon);
        let monday = week.first_day();
        let sunday = week.last_day();
        WeekCalendar {
            week_number,
            monday,
            sunday,
        }
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

impl std::fmt::Display for WeekCalendar {
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

impl From<WeekCalendar> for Row {
    fn from(value: WeekCalendar) -> Self {
        row![value.week_number(), value.monday, value.sunday]
    }
}

impl From<NaiveDate> for WeekCalendar {
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
        let actual = WeekCalendar::new(ValidatedDate::from_ymd(2023, 6, 30).unwrap());

        assert_eq!(26, actual.week_number());
        assert_eq!(NaiveDate::from_ymd_opt(2023, 6, 26).unwrap(), actual.monday);
        assert_eq!(NaiveDate::from_ymd_opt(2023, 7, 2).unwrap(), actual.sunday);
    }

    #[test]
    fn new_with_calendar_single_week_1_1_2020() {
        let actual = WeekCalendar::new(ValidatedDate::from_ymd(2020, 1, 1).unwrap());

        assert_eq!(1, actual.week_number());
        assert_eq!(
            NaiveDate::from_ymd_opt(2019, 12, 30).unwrap(),
            actual.monday
        );
        assert_eq!(NaiveDate::from_ymd_opt(2020, 1, 5).unwrap(), actual.sunday);
    }

    #[test]
    fn new_within_months() {
        let actual = WeekCalendar::all_new_in_month(ValidatedDate::from_ymd(2023, 6, 30).unwrap());
        insta::assert_debug_snapshot!(actual);
    }

    #[test]
    fn new_within_months_from_earlier_year() {
        let actual = WeekCalendar::all_new_in_month(ValidatedDate::from_ymd(2023, 1, 1).unwrap());
        insta::assert_debug_snapshot!(actual);
    }

    #[test]
    fn new_within_months_at_end_of_year_2023() {
        let actual = WeekCalendar::all_new_in_month(NaiveDate::from_ymd_opt(2023, 12, 29).unwrap());
        insta::assert_debug_snapshot!(actual);
    }

    #[test]
    fn new_within_months_at_end_of_year_2022() {
        let actual = WeekCalendar::all_new_in_month(NaiveDate::from_ymd_opt(2022, 12, 10).unwrap());
        insta::assert_debug_snapshot!(actual);
    }

    #[test]
    fn last_day_current_month() {
        let current_day = NaiveDate::from_ymd_opt(2020, 1, 14).unwrap();
        let last_day_of_current_month = WeekCalendar::last_day_current_month(current_day);
        let expected = NaiveDate::from_ymd_opt(2020, 1, 31).unwrap();
        assert_eq!(expected, last_day_of_current_month);
    }

    #[test]
    fn all_weeks_in_year_1992() {
        let date: ValidatedYear = 1992.try_into().unwrap();
        let result = WeekCalendar::all_new_in_year(date);
        insta::assert_debug_snapshot!(result);
    }

    #[test]
    fn week_number() {
        fn assert_case(week_number: ValidatedWeekNumber, year: ValidatedYear, expected: String) {
            use pretty_assertions::assert_eq;
            let actual = WeekCalendar::new_week_number(week_number, year).to_string();
            assert_eq!(
                expected, actual,
                "week_number: {:?}\nyear: {:?}",
                week_number, year
            );
        }
        assert_case(
            36.into(),
            2024.try_into().unwrap(),
            "36 2024-09-02 2024-09-08".to_owned(),
        );
        assert_case(
            52.into(),
            2024.try_into().unwrap(),
            "52 2024-12-23 2024-12-29".to_owned(),
        );
        assert_case(
            53.into(),
            2024.try_into().unwrap(),
            "52 2024-12-23 2024-12-29".to_owned(),
        );
        assert_case(
            53.into(),
            2026.try_into().unwrap(),
            "53 2026-12-28 2027-01-03".to_owned(),
        );
        assert_case(
            6.into(),
            2023.try_into().unwrap(),
            "6 2023-02-06 2023-02-12".to_owned(),
        );
    }
    #[test]
    fn between_week_numbers() {
        fn assert_case(start: u32, end: u32, year: u32, expected: &[&str]) {
            use pretty_assertions::assert_eq;
            let actual = WeekCalendar::between_week_numbers(
                start.into(),
                end.into(),
                year.try_into().unwrap(),
            )
            .as_slice()
            .into_iter()
            .map(ToString::to_string)
            .collect::<Vec<String>>();

            assert_eq!(
                expected, &actual,
                "start: {}, end: {}, year: {}",
                start, end, year
            );
        }

        assert_case(
            1,
            5,
            2024,
            &[
                "1 2024-01-01 2024-01-07",
                "2 2024-01-08 2024-01-14",
                "3 2024-01-15 2024-01-21",
                "4 2024-01-22 2024-01-28",
                "5 2024-01-29 2024-02-04",
            ],
        );
        assert_case(2, 2, 2024, &["2 2024-01-08 2024-01-14"]);
        assert_case(
            6,
            8,
            2023,
            &[
                "6 2023-02-06 2023-02-12",
                "7 2023-02-13 2023-02-19",
                "8 2023-02-20 2023-02-26",
            ],
        );
    }
}
