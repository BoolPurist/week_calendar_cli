use clap::Args;
use date_validation_types::{ValidatedMonth, ValidatedYear};
use derive_getters::Getters;

#[derive(Args, Debug, Getters)]
pub struct GivenMonth {
    #[arg(value_parser = super::to_validated_month)]
    /// Month of a date. Must be between 1 and 12.
    month: Option<ValidatedMonth>,
    #[arg(value_parser = super::to_validated_year)]
    /// Day of a date. Must be between 1 and 31
    year: Option<ValidatedYear>,
}
