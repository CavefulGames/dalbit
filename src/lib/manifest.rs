use anyhow::Result;
use indexmap::IndexMap;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::path::{Path, PathBuf};
use tokio::fs;

use crate::{polyfill::Polyfill, TargetVersion};

#[async_trait::async_trait]
pub trait WritableManifest: Send + Sized + Serialize + DeserializeOwned {
    #[inline]
    async fn from_file(path: impl Into<PathBuf> + Send) -> Result<Self> {
        let content = fs::read_to_string(path.into()).await?;

        Ok(toml::from_str(content.as_str())?)
    }

    #[inline]
    async fn write(&self, path: impl Into<PathBuf> + Send) -> Result<()> {
        fs::write(path.into(), toml::to_string(self)?).await?;

        Ok(())
    }
}

/// Manifest for dalbit transpiler. This is a writable manifest.
#[derive(Debug, Deserialize, Serialize)]
pub struct Manifest {
    input: PathBuf,
    output: PathBuf,
    file_extension: Option<String>,
    target_version: TargetVersion,
    pub minify: bool,
    modifiers: IndexMap<String, bool>,
    polyfill: Polyfill,
}

impl Default for Manifest {
    fn default() -> Self {
        Self {
            input: Path::new("input.luau").to_owned(),
            output: Path::new("output.lua").to_owned(),
            file_extension: Some("lua".to_owned()),
            target_version: TargetVersion::Lua53,
            minify: true,
            modifiers: IndexMap::new(),
            polyfill: Polyfill::default(),
        }
    }
}

impl WritableManifest for Manifest {}

impl Manifest {
    #[inline]
    pub fn input(&self) -> &PathBuf {
        &self.input
    }

    #[inline]
    pub fn output(&self) -> &PathBuf {
        &self.output
    }

    #[inline]
    pub fn file_extension(&self) -> &Option<String> {
        &self.file_extension
    }

    #[inline]
    pub fn modifiers(&self) -> &IndexMap<String, bool> {
        &self.modifiers
    }

    #[inline]
    pub fn target_version(&self) -> &TargetVersion {
        &self.target_version
    }

    #[inline]
    pub fn polyfill(&self) -> &Polyfill {
        &self.polyfill
    }
}
