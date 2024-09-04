pub const MANUAL_NAME: &str = "MANUAL.txt";

pub mod cli;
pub mod generate_manual;
pub type AppResult<T = ()> = Result<T, Box<dyn std::error::Error>>;
pub mod build_readme;
pub mod template;
