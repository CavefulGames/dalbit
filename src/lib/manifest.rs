use anyhow::{Context, Result};
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use tokio::fs;

use crate::{polyfill::Polyfill, TargetVersion};

pub const DEFAULT_MANIFEST_PATH: &str = "dalbit.toml";

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
    #[serde(skip)]
    path: PathBuf,
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
            path: Path::new(DEFAULT_MANIFEST_PATH).to_owned(),
        }
    }
}

impl Manifest {
    /// Load manifest from file.
    pub async fn from_file(path: impl Into<PathBuf>) -> Result<Self> {
        let path = path.into();
        let manifest = fs::read_to_string(&path).await?;
        let mut manifest: Manifest = toml::from_str(&manifest)
            .with_context(|| format!("Failed to parse manifest file: {:?}", path))?;
        manifest.path = path;
        Ok(manifest)
    }

    /// Write manifest to file.
    pub async fn write(&self, path: impl Into<PathBuf>) -> Result<()> {
        fs::write(path.into(), toml::to_string(self)?).await?;
        Ok(())
    }

    #[inline]
    pub fn input(&self) -> PathBuf {
        let path = self.path.parent().unwrap().join(&self.input);
        log::debug!("manifest input path: {:?}", path);
        path
    }

    #[inline]
    pub fn output(&self) -> PathBuf {
        let path = self.path.parent().unwrap().join(&self.output);
        log::debug!("manifest output path: {:?}", path);
        path
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
