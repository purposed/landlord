use std::path::PathBuf;

use rood::CausedResult;

use crate::lease::BuildConfig;
use crate::BuildMode;

// Builder defines a backend for building specific stacks.
pub trait Builder {
    fn build(
        &self,
        project_path: &PathBuf,
        cfg: &BuildConfig,
        mode: &BuildMode,
    ) -> CausedResult<PathBuf>;
    fn clean(&self) -> CausedResult<()>;
}

pub trait Validator {
    fn validate(&self, project_path: &PathBuf) -> CausedResult<()>;
}
