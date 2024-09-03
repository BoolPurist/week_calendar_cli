use clap::Args;
use date_validation_types::units::ValidatedYear;

use crate::validated_week_number::ValidatedWeekNumber;

#[derive(Args, Debug)]
/// Converts given week numbers into their respective week calendars
/// Value for week numbers are between 1 and 53.
/// Note: Some years only have 52 week numbers.
/// If for such year 53 is given then that number is treated as 52.
pub struct WeekNumberCliParams {
    /// Single week number to convert if no end week number is given.
    start: ValidatedWeekNumber,
    #[arg(short, long,value_parser = super::to_validated_year)]
    /// Given year. If omitted then the current year is assumed.
    year: Option<ValidatedYear>,
    /// End week number within a year.
    end: Option<ValidatedWeekNumber>,
}

impl WeekNumberCliParams {
    pub fn year(&self) -> Option<ValidatedYear> {
        self.year
    }

    pub fn end(&self) -> Option<ValidatedWeekNumber> {
        self.end
    }

    pub fn start(&self) -> ValidatedWeekNumber {
        self.start
    }
}
