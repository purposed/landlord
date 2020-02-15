use std::fs;
use std::path::PathBuf;

use rood::sys::{file, Architecture, Platform};
use rood::{Cause, CausedResult, Error};

use semver::Version;

use serde::{Deserialize, Serialize};

use crate::ProjectStack;
use std::io::Write;

fn default_checksum() -> bool {
    false
}

fn default_artifact_dir() -> String {
    String::from("artifacts")
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct BuildConfig {
    pub name: Option<String>,
    pub architecture: Architecture,
    pub platform: Platform,
    pub src_path: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Validation {
    pub name: String,
    pub command: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ArtifactConfig {
    pub name: String,

    #[serde(default = "default_checksum")]
    pub include_checksum: bool,

    pub path_template: String,
    pub target_name_template: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Lease {
    #[serde(skip)]
    path: PathBuf,

    pub name: String,
    pub version: Version,
    pub stack: ProjectStack,

    #[serde(default = "default_artifact_dir")]
    pub artifact_directory: String,

    #[serde(rename = "build")]
    pub builds: Vec<BuildConfig>,

    #[serde(rename = "artifact")]
    pub artifacts: Vec<ArtifactConfig>,

    #[serde(rename = "validation")]
    pub additional_validations: Option<Vec<Validation>>,
}

impl Lease {
    pub fn new(path: PathBuf) -> CausedResult<Lease> {
        file::ensure_exists(&path)?;
        let mut l: Lease = toml::from_str(&fs::read_to_string(&path)?).or_else(|e| {
            Err(Error::new(
                Cause::SerializationError,
                &format!("Could not read lease: {}", e),
            ))
        })?;
        l.path = path;

        Ok(l)
    }

    pub fn save(&self) -> CausedResult<()> {
        let mut file_handle = fs::File::create(&self.path)?;
        let raw = toml::to_string_pretty(&self).or_else(|e| {
            Err(Error::new(
                Cause::SerializationError,
                &format!("Could not write lease: {}", e),
            ))
        })?;

        file_handle.write_all(raw.as_bytes())?;
        Ok(())
    }
}
