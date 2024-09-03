use std::process::ExitCode;

use chrono::{Datelike, Local, NaiveDate};
use clap::Parser;
use date_validation_types::prelude::*;
use week_calendar::cli::{CliApp, SubCommands};
use week_calendar::{calendar_week_operations, CalendarWeekForDisplay, WeekCalendar};

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

fn handle_sub_command(cli_args: CliApp) -> Result<String, Box<dyn std::error::Error>> {
    let weeks = match cli_args.sub_command {
        SubCommands::Today => {
            let today = WeekCalendar::today();
            vec![today]
        }
        SubCommands::Date(date) => {
            let date: ValidatedDate = date.try_into()?;
            let date = WeekCalendar::new(date);
            vec![date]
        }
        SubCommands::Month(cli_month) => {
            let now = Local::now().naive_local().date();
            let month = cli_month.month().map(u32::from).unwrap_or(now.month());
            let year = cli_month.year().map(u32::from).unwrap_or(now.year() as u32);
            {
                let naive_date = NaiveDate::from_ymd_opt(year as i32, month, now.day()).unwrap();
                WeekCalendar::all_new_in_month(naive_date)
            }
        }
        SubCommands::Year(given_year) => {
            let dates: Vec<WeekCalendar> = given_year
                .year()
                .map(WeekCalendar::all_new_in_year)
                .unwrap_or_else(|| {
                    let year = Local::now().naive_local().date().year();
                    let now: ValidatedYear = (year as u32)
                        .try_into()
                        .expect("Year comes from a valid date");
                    WeekCalendar::all_new_in_year(now)
                });

            dates
        }
        SubCommands::WeekNumber(week_number) => {
            calendar_week_operations::operate_on_week_number(week_number)
        }
    }
    .into_iter()
    .map(CalendarWeekForDisplay::from);

    if cli_args.for_machine {
        Ok(string_for_parsing(weeks))
    } else {
        let table = week_calendar::week_calendar_into_table(weeks);
        Ok(table.to_string())
    }
}

fn string_for_parsing(weeks: impl Iterator<Item = CalendarWeekForDisplay>) -> String {
    weeks
        .map(|week| week.to_string())
        // Have to collect to get access to join
        .collect::<Vec<String>>()
        .join("\n")
}
