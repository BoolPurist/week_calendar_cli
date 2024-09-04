use std::path::Path;

use clap::Parser;
use xshell::Shell;
use xtask::{
    build_readme,
    cli::{BuildCli, PossibleSubCommands},
    generate_manual, AppResult,
};

fn main() -> AppResult {
    let cli = BuildCli::parse();
    let project_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("..");
    std::env::set_current_dir(&project_path)?;
    let shell = Shell::new()?;
    match cli.subcommand() {
        PossibleSubCommands::GenerateManual => generate_manual::generate(shell, &project_path),
        PossibleSubCommands::BuildReadme => build_readme::build(shell, &project_path),
    }
}
