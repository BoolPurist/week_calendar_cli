use clap::Args;
use date_validation_types::{
    InvalidDate, ValidatedDate, ValidatedDay, ValidatedMonth, ValidatedYear,
};

#[derive(Debug, Args)]
/// Shows calendar week of a given date.
/// Given date must be a valid date. Example: 30.02.2023 is not valid.
pub struct GivenDate {
    #[arg(value_parser = super::to_validated_year)]
    /// Year of a date
    year: ValidatedYear,
    #[arg(value_parser = super::to_validated_month)]
    /// Month of a date. Must be between 1 and 12.
    month: ValidatedMonth,
    #[arg(value_parser = super::to_validated_day)]
    /// Day of a date. Must be between 1 and 31
    day: ValidatedDay,
}

impl TryFrom<GivenDate> for ValidatedDate {
    type Error = InvalidDate;
    fn try_from(value: GivenDate) -> Result<Self, Self::Error> {
        ValidatedDate::new(value.year, value.month, value.day)
    }
}
