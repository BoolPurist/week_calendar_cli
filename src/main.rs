use std::process::ExitCode;

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
    let output = match cli_args.sub_command {
        SubCommands::Today => {
            let today = WeekWithCalendarNumber::today();
            let table = week_calendar::week_calendar_into_table(vec![today].into_iter());
            table.to_string()
        }
        SubCommands::Date(date) => {
            let date: ValidatedDate = date.try_into()?;
            let date = WeekWithCalendarNumber::new(date);
            let table = week_calendar::week_calendar_into_table(vec![date].into_iter());
            table.to_string()
        }
    };
    Ok(output)
}
