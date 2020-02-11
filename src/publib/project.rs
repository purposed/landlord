use std::path::{Path, PathBuf};

use rood::sys::file;
use rood::CausedResult;

use crate::git::Repository;
use crate::{constants, Lease};

#[derive(Debug)]
pub struct Project {
    pub path: PathBuf,
    pub lease: Lease,
    pub repository: Repository,
}

impl Project {
    pub fn new<T>(root_dir: T) -> CausedResult<Project>
    where
        T: AsRef<Path>,
    {
        file::ensure_exists(root_dir.as_ref())?;

        let lease = Lease::new(root_dir.as_ref().join(constants::LEASE_FILE_NAME))?;
        let repository = Repository::open(root_dir.as_ref())?;

        Ok(Project {
            lease,
            path: PathBuf::from(root_dir.as_ref()),
            repository,
        })
    }
}
