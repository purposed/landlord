use std::path::PathBuf;

use rood::CausedResult;

use crate::lease::{BuildConfig, Validation};
use crate::{BuildMode, Project};
use semver::Version;

// Builder defines a backend for building specific stacks.
pub trait Builder {
    fn build(
        &self,
        project: &Project,
        cfg: &BuildConfig,
        mode: &BuildMode,
    ) -> CausedResult<PathBuf>;
    fn clean(&self) -> CausedResult<()>;
}

pub trait Validator {
    fn get_default_validations(&self) -> Vec<Validation>;
}

pub trait Bumper {
    fn bump_all(
        &self,
        project: &Project,
        old_version: &Version,
        new_version: &Version,
    ) -> CausedResult<()>;
}
