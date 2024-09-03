use date_validation_types::units::ValidatedYear;

use crate::{chrono_utils, cli::WeekNumberCliParams, WeekCalendar};

pub type WeeksOfCalendar = Vec<WeekCalendar>;

pub fn operate_on_week_number(given_week_number: WeekNumberCliParams) -> WeeksOfCalendar {
    fn current_year() -> ValidatedYear {
        chrono_utils::get_current_year()
    }

    let start = given_week_number.start();

    match (given_week_number.year(), given_week_number.end()) {
        (None, None) => {
            vec![WeekCalendar::new_week_number(start, current_year())]
        }
        (Some(year), None) => {
            vec![WeekCalendar::new_week_number(start, year)]
        }
        (None, Some(end)) => WeekCalendar::between_week_numbers(start, end, current_year()),
        (Some(year), Some(end)) => WeekCalendar::between_week_numbers(start, end, year),
    }
}
