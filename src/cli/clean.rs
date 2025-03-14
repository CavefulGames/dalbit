use std::process::ExitCode;

use anyhow::Result;
use clap::Parser;
use dalbit_core::polyfill;
use url::Url;

/// Clean dalbit polyfills cache
#[derive(Debug, Clone, Parser)]
pub struct CleanCommand {
    repo: Option<Url>,
}

impl CleanCommand {
    pub async fn run(self) -> Result<ExitCode> {
        if let Some(repo) = self.repo {
            polyfill::clean_cache(&repo).await?;
        } else {
            polyfill::clean_cache_all().await?;
        }

        println!("Successfully cleaned polyfill caches");

        return Ok(ExitCode::SUCCESS);
    }
}
