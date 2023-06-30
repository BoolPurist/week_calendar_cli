use prettytable::{row, Row, Table};

pub mod cli;
mod week_with_calendar_number;
pub use week_with_calendar_number::WeekWithCalendarNumber;

pub fn week_calendar_into_table(weeks: impl Iterator<Item = WeekWithCalendarNumber>) -> Table {
    let mut table = Table::new();
    table.add_row(row!["Week Number", "From", "To"]);

    for next in weeks {
        let next_row: Row = next.into();
        table.add_row(next_row);
    }

    table
}
