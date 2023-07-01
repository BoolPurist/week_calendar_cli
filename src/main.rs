use std::process::ExitCode;

use chrono::{Datelike, Local, NaiveDate};
use clap::Parser;
use date_validation_types::{InvalidDate, ValidatedDate};
use week_calendar::cli::{CliApp, SubCommands};
use week_calendar::WeekWithCalendarNumber;
fn main() -> ExitCode {
    let cli_args = CliApp::parse();
    let output = handle_sub_command(cli_args);
    match output {
        Ok(table) => {
            println!("{}", table);
            ExitCode::SUCCESS
        }

        Err(error) => {
            println!("Error: {}", error);
            ExitCode::FAILURE
        }
    }
}

fn handle_sub_command(cli_args: CliApp) -> Result<String, InvalidDate> {
    let table = match cli_args.sub_command {
        SubCommands::Today => {
            let today = WeekWithCalendarNumber::today();
            week_calendar::week_calendar_into_table(vec![today].into_iter())
        }
        SubCommands::Date(date) => {
            let date: ValidatedDate = date.try_into()?;
            let date = WeekWithCalendarNumber::new(date);
            week_calendar::week_calendar_into_table(vec![date].into_iter())
        }
        SubCommands::Month(cli_month) => {
            let now = Local::now().naive_local().date();
            let month = cli_month.month().map(u32::from).unwrap_or(now.month());
            let year = cli_month.year().map(u32::from).unwrap_or(now.year() as u32);
            let date = {
                let naive_date = NaiveDate::from_ymd_opt(year as i32, month, now.day()).unwrap();
                WeekWithCalendarNumber::all_new_in_month(naive_date)
            };

            week_calendar::week_calendar_into_table(date.into_iter())
        }
    };
    Ok(table.to_string())
}
