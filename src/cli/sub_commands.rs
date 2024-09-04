use clap::Subcommand;

use crate::macros::*;

use super::{given_month::GivenMonth, GivenDate, GivenYear, WeekNumberCliParams};

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
    #[command(visible_alias = "w", after_long_help = include_str!(PATH_TO_DOCUMENTATION!(=>"sub_command_week_number_example_help.txt")))]
    WeekNumber(WeekNumberCliParams),
}
// concat!("..", PATH_SEPARATOR!() , "..", PATH_SEPARATOR!(), "resources", PATH_SEPARATOR!(),"documentation", PATH_SEPARATOR!(), "sub_command_week_number_example_help.txt"  )
