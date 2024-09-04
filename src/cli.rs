pub use given_date::GivenDate;
pub use given_week_number::WeekNumberCliParams;
pub use given_year::GivenYear;
pub use sub_commands::SubCommands;

mod given_date;
mod given_month;
mod given_week_number;
mod given_year;
mod sub_commands;

use clap::{CommandFactory, Parser};
use date_validation_types::units::{
    InvalidDay, InvalidMonth, InvalidYear, ValidatedDay, ValidatedMonth, ValidatedYear,
};

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

impl CliApp {
    pub fn get_all_subcommand_names() -> Vec<String> {
        CliApp::command()
            .get_subcommands()
            .map(|next| next.get_name().to_string())
            .into_iter()
            .collect()
    }
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
