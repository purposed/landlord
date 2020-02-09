use std::path::{Path, PathBuf};

use rood::sys::file;
use rood::CausedResult;

use crate::{constants, Lease};

#[derive(Debug)]
pub struct Project {
    pub path: PathBuf,
    pub lease: Lease,
}

impl Project {
    pub fn new(root_dir: &str) -> CausedResult<Project> {
        file::ensure_exists(root_dir)?;

        let lease = Lease::new(Path::new(root_dir).join(constants::LEASE_FILE_NAME))?;

        Ok(Project {
            lease,
            path: PathBuf::from(root_dir),
        })
    }
}
