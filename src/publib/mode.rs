use std::convert::TryFrom;

use clap::ArgMatches;
use rood::{Cause, CausedResult, Error};

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

    fn try_from(other: String) -> CausedResult<BuildMode> {
        match other.as_ref() {
            "debug" => Ok(BuildMode::Debug),
            "release" => Ok(BuildMode::Release),
            _ => Err(Error::new(
                Cause::InvalidData,
                &format!("Cannot parse '{}' as BuildMode", other),
            )),
        }
    }
}
