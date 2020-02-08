use std::collections::HashMap;
use std::path::{Path, PathBuf};

use rood::{Cause, CausedResult};

use crate::lease::BuildConfig;
use crate::Project;

pub enum BuildMode {
    Debug,
    Release,
}

// Builder defines a backend for building specific stacks.
pub trait Builder {
    fn build(&self, path: &PathBuf, cfg: &BuildConfig, mode: &BuildMode) -> CausedResult<PathBuf>;
    fn clean(&self) -> CausedResult<()>;
}
