use prettytable::{row, Row};

use crate::WeekCalendar;

pub struct CalendarWeekForDisplay {
    week: u32,
    start_week: String,
    end_week: String,
}

impl From<WeekCalendar> for CalendarWeekForDisplay {
    fn from(value: WeekCalendar) -> Self {
        let week = value.week_number();
        let start_week = (value.monday()).to_string();
        let end_week = (value.sunday()).to_string();

        Self {
            week,
            start_week,
            end_week,
        }
    }
}

impl From<CalendarWeekForDisplay> for Row {
    fn from(value: CalendarWeekForDisplay) -> Self {
        row![value.week, value.start_week, value.end_week]
    }
}

impl std::fmt::Display for CalendarWeekForDisplay {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}", self.week, self.start_week, self.end_week)
    }
}
