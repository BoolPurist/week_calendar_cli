use date_validation_types::units::ValidatedYear;

use crate::{
    chrono_utils, cli::WeekNumberCliParams, validated_week_number::ValidatedWeekNumber,
    WeekCalendar,
};

pub type WeeksOfCalendar = Vec<WeekCalendar>;

pub fn operate_on_week_number(given_week_number: WeekNumberCliParams) -> WeeksOfCalendar {
    fn current_year() -> ValidatedYear {
        chrono_utils::get_current_year()
    }

    fn may_swap_if_end_greater_start(
        start: ValidatedWeekNumber,
        end: ValidatedWeekNumber,
    ) -> (ValidatedWeekNumber, ValidatedWeekNumber) {
        if start > end {
            (end, start)
        } else {
            (start, end)
        }
    }

    let start = given_week_number.start();

    let year = given_week_number.year().unwrap_or_else(current_year);
    match given_week_number.end() {
        None => {
            vec![WeekCalendar::new_week_number(start, year)]
        }
        Some(end) => {
            let (start, end) = may_swap_if_end_greater_start(start, end);
            WeekCalendar::between_week_numbers(start, end, year)
        }
    }
}
