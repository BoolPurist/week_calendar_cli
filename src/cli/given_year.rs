use clap::Args;
use date_validation_types::ValidatedYear;
use derive_getters::Getters;

#[derive(Args, Debug, Getters)]
/// List all calendar weeks in a given year. If no year is provided then the current year is
/// assumed
pub struct GivenYear {
    #[arg(value_parser = super::to_validated_year)]
    /// Day of a date. Must be between 1 and 31
    year: Option<ValidatedYear>,
}