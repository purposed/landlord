use std::path::PathBuf;

use anyhow::Result;

use semver::Version;

use crate::lease::{BuildConfig, Validation};
use crate::{BuildMode, Project};

// Builder defines a backend for building specific stacks.
pub trait Builder {
    fn build(&self, project: &Project, cfg: &BuildConfig, mode: &BuildMode) -> Result<PathBuf>;
    fn clean(&self) -> Result<()>;
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
    ) -> Result<()>;
}
