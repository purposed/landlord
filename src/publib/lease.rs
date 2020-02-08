use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

use rood::sys::{file, Architecture, Platform};
use rood::{Cause, CausedResult, Error};

use semver::Version;

use serde::Deserialize;

use toml;

use crate::{Project, ProjectStack};
use std::hash::Hash;

fn default_checksum() -> bool {
    false
}

#[derive(Debug, Clone, Deserialize)]
pub struct BuildConfig {
    pub architecture: Architecture,
    pub platform: Platform,
}

#[derive(Debug, Deserialize)]
pub struct ArtifactConfig {
    #[serde(default = "default_checksum")]
    pub include_checksum: bool,

    pub path: String,
    pub platforms: Vec<Platform>,
}

#[derive(Debug, Deserialize)]
pub struct Lease {
    pub name: String,
    pub version: Version,
    pub stack: ProjectStack,

    #[serde(rename = "build")]
    pub builds: Vec<BuildConfig>,
    pub artifacts: HashMap<String, ArtifactConfig>,
}

impl Lease {
    pub fn new(path: PathBuf) -> CausedResult<Lease> {
        file::ensure_exists(&path)?;
        let l: Lease = toml::from_str(&fs::read_to_string(&path)?).or_else(|e| {
            Err(Error::new(
                Cause::SerializationError,
                &format!("Could not read lease: {}", e),
            ))
        })?;

        Ok(l)
    }
}
