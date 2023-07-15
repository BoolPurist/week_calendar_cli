use clap::{Parser, Subcommand};
use date_validation_types::{
    InvalidDay, InvalidMonth, InvalidYear, ValidatedDay, ValidatedMonth, ValidatedYear,
};

mod given_date;
mod given_month;
mod given_year;
pub use given_date::GivenDate;
pub use given_year::GivenYear;

use self::given_month::GivenMonth;

#[derive(Parser)]
#[command(author = "BoolPurist", version)]
/// Lists calendar week at a certain time or in a monthly/yearly interveral.
///
/// Source code and example of useage can be found at: https://github.com/BoolPurist/week_calendar_cli
pub struct CliApp {
    /// Data entry about week calendar are outputed with spaces between.
    /// This is intended to make parsing of the date easier.
    #[arg(short, long, env = "WEEK_CALENDAR_FOR_MACHINE")]
    pub for_machine: bool,
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
    #[command(visible_alias = "m")]
    Month(GivenMonth),
    #[command(visible_alias = "y")]
    Year(GivenYear),
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
