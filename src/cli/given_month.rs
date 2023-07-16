use clap::Args;
use date_validation_types::units::{ValidatedMonth, ValidatedYear};
use derive_getters::Getters;

#[derive(Args, Debug, Getters)]
/// List all calendar weeks in a given month in a given year.
pub struct GivenMonth {
    #[arg(value_parser = super::to_validated_month)]
    /// Month of a date. Must be between 1 and 12. If not provided then the current month and
    /// year are assumed.
    month: Option<ValidatedMonth>,
    #[arg(value_parser = super::to_validated_year)]
    /// Day of a date. Must be between 1 and 31. If not provided then the current year is
    /// assumed.
    year: Option<ValidatedYear>,
}
