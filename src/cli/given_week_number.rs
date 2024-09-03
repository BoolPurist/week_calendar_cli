use clap::Args;
use date_validation_types::units::ValidatedYear;

use crate::validated_week_number::ValidatedWeekNumber;

#[derive(Args, Debug)]
pub struct GivenWeekNumber {
    start: ValidatedWeekNumber,
    #[arg(short, long,value_parser = super::to_validated_year)]
    /// Given year. If omited then the current year is assumed.
    year: Option<ValidatedYear>,
    end: Option<ValidatedWeekNumber>,
}

impl GivenWeekNumber {
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
