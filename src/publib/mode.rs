use std::convert::TryFrom;

use anyhow::{anyhow, Error, Result};

use clap::ArgMatches;

const DEFAULT_BUILD_MODE: BuildMode = BuildMode::Release;

#[derive(Debug)]
pub enum BuildMode {
    Debug,
    Release,
}

impl BuildMode {
    pub fn get(matches: &ArgMatches) -> BuildMode {
        if let Some(mode) = matches.value_of("mode") {
            BuildMode::try_from(String::from(mode)).unwrap_or(DEFAULT_BUILD_MODE)
        } else {
            DEFAULT_BUILD_MODE
        }
    }
}

impl TryFrom<String> for BuildMode {
    type Error = Error;

    fn try_from(other: String) -> Result<BuildMode> {
        match other.as_ref() {
            "debug" => Ok(BuildMode::Debug),
            "release" => Ok(BuildMode::Release),
            _ => Err(anyhow!("Cannot parse {} as BuildMode", other)),
        }
    }
}
