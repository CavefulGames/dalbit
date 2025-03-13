use std::{
    path::{Path, PathBuf},
    process::ExitCode,
};

use anyhow::{anyhow, Result};
use clap::Parser;
use dalbit_core::manifest::Manifest;

use dalbit_core::manifest::DEFAULT_MANIFEST_PATH;

/// Initialize dalbit manifest file
#[derive(Debug, Clone, Parser)]
pub struct InitCommand {
    /// Path to initialize the manifest file
    path: Option<String>,
}

impl InitCommand {
    pub async fn run(self) -> Result<ExitCode> {
        let mut buffer = PathBuf::new();
        let config_path = self
            .path
            .as_deref()
            .map(|p| {
                buffer = Path::new(p).join("dalbit.toml");
                buffer.as_path()
            })
            .unwrap_or_else(|| Path::new(DEFAULT_MANIFEST_PATH));
        log::debug!("config path: {:?}", config_path);
        if config_path.exists() {
            return Err(anyhow!("Manifest has already been initialized"));
        }
        let manifest = Manifest::default();
        manifest.write(config_path).await?;

        println!("Initialized dalbit manifest");

        return Ok(ExitCode::SUCCESS);
    }
}
