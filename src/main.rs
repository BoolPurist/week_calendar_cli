use std::process::ExitCode;

use chrono::{Datelike, Local, NaiveDate};
use clap::Parser;
use date_validation_types::prelude::*;
use week_calendar::cli::{CliApp, SubCommands};
use week_calendar::{chrono_utils, WeekCalendarDisplay, WeekCalendarNumber};

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
            let today = WeekCalendarNumber::today();
            vec![today]
        }
        SubCommands::Date(date) => {
            let date: ValidatedDate = date.try_into()?;
            let date = WeekCalendarNumber::new(date);
            vec![date]
        }
        SubCommands::Month(cli_month) => {
            let now = Local::now().naive_local().date();
            let month = cli_month.month().map(u32::from).unwrap_or(now.month());
            let year = cli_month.year().map(u32::from).unwrap_or(now.year() as u32);
            {
                let naive_date = NaiveDate::from_ymd_opt(year as i32, month, now.day()).unwrap();
                WeekCalendarNumber::all_new_in_month(naive_date)
            }
        }
        SubCommands::Year(given_year) => {
            let dates: Vec<WeekCalendarNumber> = given_year
                .year()
                .map(WeekCalendarNumber::all_new_in_year)
                .unwrap_or_else(|| {
                    let year = Local::now().naive_local().date().year();
                    let now: ValidatedYear = (year as u32).try_into().unwrap();
                    WeekCalendarNumber::all_new_in_year(now)
                });

            dates
        }
        SubCommands::WeekNumber(week_number) => match (week_number.year(), week_number.end()) {
            (None, None) => {
                let current_year = chrono_utils::get_current_year();
                vec![
                    WeekCalendarNumber::new_week_number(week_number.start(), current_year).unwrap(),
                ]
            }
            (Some(year), None) => {
                vec![WeekCalendarNumber::new_week_number(week_number.start(), year).unwrap()]
            }
            (None, Some(_)) => todo!(),
            (Some(_), Some(_)) => todo!(),
        },
    }
    .into_iter()
    .map(WeekCalendarDisplay::from);

    if cli_args.for_machine {
        let rows = weeks
            .map(|week| week.to_string())
            // Have to collect to get access to join
            .collect::<Vec<String>>()
            .join("\n");
        Ok(rows)
    } else {
        let table = week_calendar::week_calendar_into_table(weeks);
        Ok(table.to_string())
    }
}
