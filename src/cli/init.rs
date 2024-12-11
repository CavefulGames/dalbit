use std::process::ExitCode;

use anyhow::Result;
use clap::Parser;
use dalbit_core::manifest::{Manifest, WritableManifest};

use crate::cli::DEFAULT_MANIFEST_PATH;

/// Initialize dalbit manifest file
#[derive(Debug, Clone, Parser)]
pub struct InitCommand {}

impl InitCommand {
    pub async fn run(self) -> Result<ExitCode> {
        let manifest = Manifest::default();
        manifest.write(DEFAULT_MANIFEST_PATH).await?;

        println!("Initialized dalbit.toml");

        return Ok(ExitCode::SUCCESS);
    }
}
