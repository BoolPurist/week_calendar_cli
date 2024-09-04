use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
pub struct BuildCli {
    #[command(subcommand)]
    subcommand: PossibleSubCommands,
}

impl BuildCli {
    pub fn subcommand(&self) -> &PossibleSubCommands {
        &self.subcommand
    }
}

#[derive(Debug, Subcommand, Clone)]
pub enum PossibleSubCommands {
    GenerateManual,
    BuildReadme,
}
