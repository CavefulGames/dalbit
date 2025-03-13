use std::process::ExitCode;

use anyhow::Result;
use clap::{Args, Parser, Subcommand};

mod clean;
mod fetch;
mod init;
mod transpile;

use clean::CleanCommand;
use fetch::FetchCommand;
use init::InitCommand;
use log::LevelFilter;
use transpile::TranspileCommand;

#[derive(Debug, Clone, Subcommand)]
pub enum CliSubcommand {
    Transpile(TranspileCommand),
    Init(InitCommand),
    Fetch(FetchCommand),
    Clean(CleanCommand),
}

#[derive(Debug, Args, Clone)]
pub struct GlobalOptions {
    /// Sets verbosity level (can be specified multiple times)
    #[arg(long, short, global(true), action = clap::ArgAction::Count)]
    verbose: u8,
}

impl GlobalOptions {
    pub fn get_log_level_filter(&self) -> LevelFilter {
        match self.verbose {
            0 => LevelFilter::Warn,
            1 => LevelFilter::Info,
            2 => LevelFilter::Debug,
            _ => LevelFilter::Trace,
        }
    }
}

/// Transpile Luau scripts
#[derive(Parser, Debug, Clone)]
#[command(version, about, long_about = None)]
pub struct Dalbit {
    #[command(flatten)]
    global_options: GlobalOptions,
    #[clap(subcommand)]
    subcommand: CliSubcommand,
}

impl Dalbit {
    pub async fn run(self) -> Result<ExitCode> {
        match self.subcommand {
            CliSubcommand::Transpile(cmd) => cmd.run().await,
            CliSubcommand::Init(cmd) => cmd.run().await,
            CliSubcommand::Fetch(cmd) => cmd.run().await,
            CliSubcommand::Clean(cmd) => cmd.run().await,
        }
    }

    pub fn get_log_level_filter(&self) -> LevelFilter {
        self.global_options.get_log_level_filter()
    }
}
