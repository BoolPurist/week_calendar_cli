use chrono::{Datelike, Days, Local, NaiveDate, Weekday};
use date_validation_types::units::ValidatedYear;

use crate::validated_week_number::ValidatedWeekNumber;

pub fn get_today_date() -> NaiveDate {
    Local::now().naive_local().date()
}

pub fn get_current_year() -> ValidatedYear {
    let year = get_today_date().year() as u32;
    ValidatedYear::new(year).unwrap()
}

pub fn calc_max_week_number_in_year(given_year: ValidatedYear) -> ValidatedWeekNumber {
    const ONE_WEEK: u64 = 7;
    const FIRST_WEEK_OF_NEXT_YEAR: u32 = 1;

    let last_date_of_given_year =
        NaiveDate::from_ymd_opt(u32::from(given_year) as i32, 12, 31).unwrap();

    let monday = last_date_of_given_year.week(Weekday::Mon).first_day();
    let might_be_in_next_year = monday.iso_week().week();
    let already_in_next_year = might_be_in_next_year == FIRST_WEEK_OF_NEXT_YEAR;

    if already_in_next_year {
        let before_first_week = monday
            .checked_sub_days(Days::new(ONE_WEEK))
            .expect(
                "Being in the next year means there is a previous year which is more than 7 days",
            )
            .iso_week()
            .week();
        ValidatedWeekNumber::new(before_first_week)
    } else {
        ValidatedWeekNumber::new(might_be_in_next_year)
    }
    .expect("End of year will end up with 52, 53 or 1. Case 1 is always followed by 52 or 53.")
}

#[cfg(test)]
mod testing {
    use super::*;
    #[test]
    fn calc_right_max_week_number_this_year() {
        fn assert_case(given_year: ValidatedYear, expected: ValidatedWeekNumber) {
            let actual = calc_max_week_number_in_year(given_year);
            assert_eq!(expected, actual, "given_year: {:?}", given_year);
        }
        assert_case(
            ValidatedYear::new(2020).unwrap(),
            ValidatedWeekNumber::new(53).unwrap(),
        );
        assert_case(
            ValidatedYear::new(2021).unwrap(),
            ValidatedWeekNumber::new(52).unwrap(),
        );
        assert_case(
            ValidatedYear::new(2024).unwrap(),
            ValidatedWeekNumber::new(52).unwrap(),
        );
        assert_case(
            ValidatedYear::new(2025).unwrap(),
            ValidatedWeekNumber::new(52).unwrap(),
        );
        assert_case(
            ValidatedYear::new(2026).unwrap(),
            ValidatedWeekNumber::new(53).unwrap(),
        );
    }
}
