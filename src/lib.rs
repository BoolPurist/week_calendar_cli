use prettytable::{row, Row, Table};

pub use week_calendar_result::WeekCalendarDisplay;
pub use week_with_calendar_number::WeekCalendarNumber;
pub mod chrono_utils;
pub mod cli;

mod validated_week_number;
mod week_calendar_result;
mod week_with_calendar_number;

pub fn week_calendar_into_table(weeks: impl IntoIterator<Item = WeekCalendarDisplay>) -> Table {
    let mut table = Table::new();

    table.add_row(row!["Week Number", "From", "To"]);

    for next in weeks {
        let next_row: Row = next.into();
        table.add_row(next_row);
    }

    table
}
