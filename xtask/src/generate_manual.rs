use std::path::Path;

use week_calendar::cli::CliApp;
use xshell::{cmd, Shell};

use crate::{AppResult, MANUAL_NAME};

pub fn generate(shell: Shell, project_path: &Path) -> AppResult {
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
