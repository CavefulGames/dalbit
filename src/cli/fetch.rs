use std::{path::Path, process::ExitCode};

use anyhow::Result;
use clap::Parser;
use dalbit_core::manifest::Manifest;

use dalbit_core::manifest::DEFAULT_MANIFEST_PATH;

/// Fetch dalbit polyfills
#[derive(Debug, Clone, Parser)]
pub struct FetchCommand {
    /// Path to the manifest file
    #[arg(long, default_value = DEFAULT_MANIFEST_PATH)]
    config: String,
}

impl FetchCommand {
    pub async fn run(self) -> Result<ExitCode> {
        let config_path = Path::new(&self.config);
        let manifest = Manifest::from_file(config_path).await?;
        let Some(polyfill) = manifest.polyfill() else {
            println!("No polyfill configured in the manifest.");
            return Ok(ExitCode::SUCCESS);
        };
        let polyfill_cache = polyfill.cache().await?;
        polyfill_cache.fetch()?;

        // TO-DO: Is fetched polyfill already latest version?

        return Ok(ExitCode::SUCCESS);
    }
}
