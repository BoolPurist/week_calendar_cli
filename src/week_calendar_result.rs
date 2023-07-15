use prettytable::{row, Row};

use crate::WeekCalendarNumber;

pub struct WeekCalendarDisplay {
    week: u32,
    start_week: String,
    end_week: String,
}

impl From<WeekCalendarNumber> for WeekCalendarDisplay {
    fn from(value: WeekCalendarNumber) -> Self {
        let week = *value.week();
        let start_week = (*value.monday()).to_string();
        let end_week = (*value.sunday()).to_string();

        Self {
            week,
            start_week,
            end_week,
        }
    }
}

impl From<WeekCalendarDisplay> for Row {
    fn from(value: WeekCalendarDisplay) -> Self {
        row![value.week, value.start_week, value.end_week]
    }
}
impl std::fmt::Display for WeekCalendarDisplay {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}", self.week, self.start_week, self.end_week)
    }
}
