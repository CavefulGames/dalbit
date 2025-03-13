use std::{path::Path, process::ExitCode};

use anyhow::Result;
use clap::Parser;
use dalbit_core::{manifest::Manifest, transpile};
use std::time::Instant;

use dalbit_core::manifest::DEFAULT_MANIFEST_PATH;

/// Transpile luau files into lua files
#[derive(Debug, Clone, Parser)]
pub struct TranspileCommand {
    /// Path to the manifest file
    #[arg(long, default_value = DEFAULT_MANIFEST_PATH)]
    config: String,
}

impl TranspileCommand {
    pub async fn run(self) -> Result<ExitCode> {
        let config_path = Path::new(&self.config);
        let process_start_time = Instant::now();

        let manifest = Manifest::from_file(config_path).await?;

        transpile::process(manifest).await?;

        let process_duration = durationfmt::to_string(process_start_time.elapsed());

        println!("Successfully transpiled in {}", process_duration);

        return Ok(ExitCode::SUCCESS);
    }
}
