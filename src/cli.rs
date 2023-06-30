use clap::{Parser, Subcommand};
use date_validation_types::{
    InvalidDay, InvalidMonth, InvalidYear, ValidatedDay, ValidatedMonth, ValidatedYear,
};

mod given_date;
pub use given_date::GivenDate;

#[derive(Parser)]
#[command(author, version, about = "Shows calendar week")]
pub struct CliApp {
    #[command(subcommand)]
    pub sub_command: SubCommands,
}

#[derive(Debug, Subcommand)]
pub enum SubCommands {
    /// Shows calendar week for today
    #[command(visible_alias = "t")]
    Today,
    #[command(visible_alias = "d")]
    Date(GivenDate),
}

pub fn to_validated_day(input: &str) -> Result<ValidatedDay, String> {
    let day: u32 = input.parse::<u32>().map_err(|e| e.to_string())?;
    let day: ValidatedDay = day.try_into().map_err(|e: InvalidDay| e.to_string())?;
    Ok(day)
}
pub fn to_validated_month(input: &str) -> Result<ValidatedMonth, String> {
    let month: u32 = input.parse::<u32>().map_err(|e| e.to_string())?;
    let month: ValidatedMonth = month.try_into().map_err(|e: InvalidMonth| e.to_string())?;
    Ok(month)
}
pub fn to_validated_year(input: &str) -> Result<ValidatedYear, String> {
    let year: u32 = input.parse::<u32>().map_err(|e| e.to_string())?;
    let year: ValidatedYear = year.try_into().map_err(|e: InvalidYear| e.to_string())?;
    Ok(year)
}
