use std::path::Path;

use week_calendar::cli::CliApp;
use xshell::{cmd, Shell};

const MANUAL_NAME: &str = "MANUAL.txt";
type AppResult<T = ()> = Result<T, Box<dyn std::error::Error>>;

fn main() -> AppResult {
    let project_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("..");
    std::env::set_current_dir(&project_path)?;
    let shell = Shell::new()?;
    let manual_path = project_path.join(MANUAL_NAME);

    let mut whole_help = cmd!(shell, "cargo r -- --help").read()?;
    let banner = format!("\n\n{}\n\n", &"-".repeat(100));
    for next_subcommand in CliApp::get_all_subcommand_names() {
        whole_help.push_str(&banner);
        let next_help = cmd!(shell, "cargo r -- {next_subcommand} --help").read()?;
        whole_help.push_str(&next_help);
    }

    shell.write_file(&manual_path, &whole_help)?;

    Ok(())
}
