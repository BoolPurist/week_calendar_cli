use prettytable::{row, Row, Table};

pub use simple_parseable_week_calendar::CalendarWeekForDisplay;
pub use week_with_calendar_number::WeekCalendar;
pub mod chrono_utils;
pub mod cli;

pub mod calendar_week_operations;
mod simple_parseable_week_calendar;
mod validated_week_number;
mod week_with_calendar_number;

pub fn week_calendar_into_table(weeks: impl IntoIterator<Item = CalendarWeekForDisplay>) -> Table {
    let mut table = Table::new();

    table.add_row(row!["Week Number", "From", "To"]);

    for next in weeks {
        let next_row: Row = next.into();
        table.add_row(next_row);
    }

    table
}
